use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::marker::ConstParamTy;
use std::mem;
use std::mem::ManuallyDrop;
use std::ops::{Index, IndexMut};
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstVec<T, const N: usize>([Option<T>; N]);

impl<T: Copy + Debug, const N: usize> ConstVec<T, N> {
    pub const fn new() -> Self {
        ConstVec([None; N])
    }

    pub const fn len(&self) -> usize {
        let mut s = 0;
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].is_some() {
                s += 1;
            }
            i += 1;
        }
        s
    }

    pub const fn push(&mut self, t: T) {
        let mut i = 0;
        while i < self.0.len() {
            let i_mut = &mut self.0[i];
            match i_mut {
                None => {
                    // T: Copy helps convince the compiler we are not running a destructor at
                    // compile time here
                    *i_mut = Some(t);
                    return
                }
                Some(_) => {
                    // Continue to iterate until possibly overflow
                }
            }
            i += 1;
        }
        // Cannot format nicer error message in const evaluation
        panic!("ConstVec overflow")
    }

    pub const fn get(&self, index: usize) -> &T {
        match &self.0[index] {
            None => {
                // Cannot format nicer error message in const evaluation
                panic!("ConstVec get() index out of bounds")
            }
            Some(stuff) => {
                return stuff
            }
        }
    }

    pub const fn get_mut(&mut self, index: usize) -> &mut T {
        match &mut self.0[index] {
            None => {
                panic!("ConstVec get_mut() index out of bounds")
            }
            Some(stuff) => {
                return stuff
            }
        }
    }
}

impl<T: Copy + Debug, const N: usize> Index<usize> for ConstVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}
impl<T: Copy + Debug, const N: usize> IndexMut<usize> for ConstVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}


/// Option does implement StructuralPartialEq, but
/// Option does not implement ConstParamTy.
/// So we should probably PR rust to make Option implement ContParamTy, but
/// until then, here we go.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum ConstOption<T> {
    None,
    Some(T)
}
impl<T: ConstParamTy> ConstParamTy for ConstOption<T> {}
impl<T: Copy> ConstOption<T> {
    pub const fn from_option(opt: Option<T>) -> Self {
        match opt {
            None => ConstOption::None,
            Some(t) => ConstOption::Some(t)
        }
    }

    pub const fn into_option(self) -> Option<T> {
        match self {
            ConstOption::None => None,
            ConstOption::Some(t) => Some(t),
        }
    }

    pub const fn is_none(&self) -> bool {
        match self {
            ConstOption::None => true,
            ConstOption::Some(_) => false,
        }
    }

    pub const fn is_some(&self) -> bool {
        match self {
            ConstOption::None => false,
            ConstOption::Some(_) => true,
        }
    }

    pub const fn expect(self, err: &'static str) -> T {
        self.into_option().expect(err)
    }
}