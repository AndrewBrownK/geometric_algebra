use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

pub enum AwaitOrClone<T: Clone> {
    InProgress(broadcast::Receiver<T>),
    Done(T)
}

impl<T: Clone> Clone for AwaitOrClone<T> {
    fn clone(&self) -> Self {
        match self {
            AwaitOrClone::InProgress(rec) => {
                AwaitOrClone::InProgress(rec.resubscribe())
            }
            AwaitOrClone::Done(arc) => {
                AwaitOrClone::Done(arc.clone())
            }
        }
    }
}

impl<T: Clone> AwaitOrClone<T> {
    pub async fn await_clone(&self) -> Result<T, RecvError> {
        let mut rec = match self {
            AwaitOrClone::InProgress(rec) => (*rec).resubscribe(),
            AwaitOrClone::Done(arc) => return Ok(arc.clone())
        };
        rec.recv().await
    }
}



#[derive(Clone)]
pub struct AsyncMap<K: Hash, V: Clone>(Arc<tokio::sync::RwLock<HashMap<K, AwaitOrClone<V>>>>);
pub enum AsyncMapResult<V> {
    AlreadyDone(V),
    ItsOnTheWay(broadcast::Receiver<V>),
    Oops(RecvError)
}
impl<V: Clone> AsyncMapResult<V> {
    pub async fn get_or_panic(self) -> V {
        match self {
            AsyncMapResult::AlreadyDone(v) => v,
            AsyncMapResult::ItsOnTheWay(mut thingy) => {
                thingy.recv().await.expect("AsyncMapResult recv error")
            }
            AsyncMapResult::Oops(err) => panic!("AsyncMapResult oopsie: {err:?}"),
            // _ => panic!()
        }
    }
}

impl<
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
> AsyncMap<K, V> {
    pub async fn get(&self, k: &K) -> Option<V> {
        let guard = self.0.read().await;
        let awaiter = guard.get(k).cloned()?;
        drop(guard);
        awaiter.await_clone().await.ok()
    }

    pub async fn get_or_create_or_panic<F: Future<Output=V> + Send + 'static>(&self, k: K, f: F) -> V {
        self.get_or_create(k, f).await.get_or_panic().await
    }
    pub async fn get_or_create<F: Future<Output=V> + Send + 'static>(&self, k: K, f: F) -> AsyncMapResult<V> {

        let read = self.0.read().await;
        match read.get(&k) {
            Some(existing) => {
                let result = existing.clone();
                drop(read);
                match result.await_clone().await {
                    Ok(v) => AsyncMapResult::AlreadyDone(v),
                    Err(err) => AsyncMapResult::Oops(err),
                }
            }
            None => {
                drop(read);
                let mut write = self.0.write().await;
                match write.entry(k.clone()) {
                    Entry::Occupied(occ) => {
                        let impl_started = occ.get().clone();
                        drop(write);
                        match impl_started.await_clone().await {
                            Ok(v) => AsyncMapResult::AlreadyDone(v),
                            Err(err) => AsyncMapResult::Oops(err),
                        }
                    }
                    Entry::Vacant(vac) => {
                        let (s, r) = broadcast::channel(1);
                        vac.insert(AwaitOrClone::InProgress(r.resubscribe()));
                        drop(write);
                        let self_ = self.clone();
                        tokio::spawn( async move {
                            let v = f.await;
                            let mut write = self_.0.write().await;
                            write.entry(k.clone())
                                .and_modify(|it| *it = AwaitOrClone::Done(v.clone()))
                                .or_insert_with(|| AwaitOrClone::Done(v.clone()));
                            drop(write);
                            let _ = s.send(v);
                        });
                        AsyncMapResult::ItsOnTheWay(r)
                    }
                }
            }
        }
    }
}