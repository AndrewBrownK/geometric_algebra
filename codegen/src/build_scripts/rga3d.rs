use crate::ast::datatype::ExpressionType;
use crate::ga;
use crate::elements::e1234;

crate::multi_vecs! { e1234;
    // Versors
    Motor      as e41, e42, e43, e1234 | e23, e31, e12, scalar;
}

/// Lengyel styled RGA of 4 dimensions representing 3 dimensions
pub fn rga3d_script() {

    // let rga3d = ga!(1 => e1, e2, e3; 0 => e4);
}


#[test]
fn test_stuff() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();
    let traits = crate::register_all! { repo;
        Wedge AntiWedge
    };
    let traits = traits.finish();

    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let impls = traits.get_impls().await;
        for i in impls {
            if let ExpressionType::Class(mv) = i.owner {
                if mv.name() == "Motor" {
                    let o = &i.other_type_params;
                    println!("{o:?}");
                    let r = &i.return_expr;
                    println!("{r:?}");
                }
            }
        }
        Some(())
    });
    result.expect("Entire script must complete")
}