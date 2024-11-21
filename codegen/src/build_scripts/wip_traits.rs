





// TODO this is just a placeholder to help me figure out powi and then powf and then sqrt
trait_impl_1_type_1_arg!(SquareImpl(builder, slf) -> MultiVector {
    let mut dyn_mv = DynamicMultiVector::zero();
    // let r = Reverse.inline(&builder, slf.clone()).await?;
    let r = slf.clone();
    for (a, a_el) in slf.elements_flat() {
        for (b, b_el) in r.elements_flat() {
            let sop = builder.ga.product(a_el, b_el);
            for p in sop.sum {
                let el = p.element;
                let f = a.clone() * b.clone() * p.coefficient;
                dyn_mv += (f, el);
            }
        }
    }
    let result = dyn_mv.construct(&builder)?;
    builder.return_expr(result)
});
trait_impl_1_type_1_arg!(AntiSquareImpl(builder, slf) -> MultiVector {
    let mut dyn_mv = DynamicMultiVector::zero();
    // let r = AntiReverse.inline(&builder, slf.clone()).await?;
    let r = slf.clone();
    for (a, a_el) in slf.elements_flat() {
        for (b, b_el) in r.elements_flat() {
            let sop = builder.ga.anti_product(a_el, b_el);
            for p in sop.sum {
                let el = p.element;
                let f = a.clone() * b.clone() * p.coefficient;
                dyn_mv += (f, el);
            }
        }
    }
    let result = dyn_mv.construct(&builder)?;
    builder.return_expr(result)
});

trait_impl_1_type_2_arg_f32!(PowfImpl(builder, slf, exp) -> MultiVector {
    let exp: FloatExpr = exp.into();
    let mut dyn_mv = DynamicMultiVector::zero();
    let r = slf.clone();
    for (a, a_el) in slf.elements_flat() {
        for (b, b_el) in r.elements_flat() {
            let sop = builder.ga.product(a_el, b_el);
            for p in sop.sum {
                let f = if a_el == b_el {
                    let a_exp = FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp.clone())), 1.0);
                    let b_exp = FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp.clone())), 1.0);
                    FloatExpr::Product(vec![(a_exp, 0.5), (b_exp, 0.5)], p.coefficient)
                } else {
                    let mut f = exp.clone() * p.coefficient * 0.5;
                    let exp_minus_one = FloatExpr::Sum(vec![(exp.clone(), 1.0)], -1.0);
                    if a_el == scalar {
                        f.mul_assign(FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                    } else {
                        f.mul_assign(a.clone());
                    }
                    if b_el == scalar {
                        f.mul_assign(FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                    } else {
                        f.mul_assign(b.clone());
                    }
                    f
                };
                dyn_mv += (f, p.element);
            }
        }
    }
    let mut result = dyn_mv.construct(&builder)?;
    if result.mv_class != slf.expr_type {
        return None
    }
    builder.return_expr(result)
});

trait_impl_1_type_2_arg_i32!(PowiImpl(builder, slf, exp) -> MultiVector {
    let exp = FloatExpr::FromInt(exp.into());
    let result = Powf.inline(&builder, slf, exp).await?;
    builder.return_expr(result)
});


/*

Base:             AntiScalar( e12345(a12345) )
Accurate Square:  AntiScalar( e12345(((a12345 ^2))) )
Accurate Cube:    AntiScalar( e12345(((a12345 ^3))) )
Accurate 4th pow: AntiScalar( e12345(((a12345 ^4))) )
Accurate 5th pow: AntiScalar( e12345(((a12345 ^5))) )
Accurate 6th pow: AntiScalar( e12345(((a12345 ^6))) )

Base:             DualNum4( e4(a4), e12345(a12345) )
Accurate Square:  DualNum4( e4((a12345 * a4 * 2)), e12345(((a12345 ^2))) )
Accurate Cube:    DualNum4( e4(((a12345 ^2) * a4 * 3)), e12345(((a12345 ^3))) )
Accurate 4th pow: DualNum4( e4(((a12345 ^3) * a4 * 4)), e12345(((a12345 ^4))) )
Accurate 5th pow: DualNum4( e4(((a12345 ^4) * a4 * 5)), e12345(((a12345 ^5))) )
Accurate 6th pow: DualNum4( e4(((a12345 ^5) * a4 * 6)), e12345(((a12345 ^6))) )

Base:             DualNum5( e5(a5), e12345(a12345) )
Accurate Square:  DualNum5( e5((a12345 * a5 * 2)), e12345(((a12345 ^2))) )
Accurate Cube:    DualNum5( e5(((a12345 ^2) * a5 * 3)), e12345(((a12345 ^3))) )
Accurate 4th pow: DualNum5( e5(((a12345 ^3) * a5 * 4)), e12345(((a12345 ^4))) )
Accurate 5th pow: DualNum5( e5(((a12345 ^4) * a5 * 5)), e12345(((a12345 ^5))) )
Accurate 6th pow: DualNum5( e5(((a12345 ^5) * a5 * 6)), e12345(((a12345 ^6))) )


Base:             DualNum321( e321(a321), e12345(a12345) )

Accurate Square:  DualNum321(
// a12345 powers: 1
// a321 powers: 1
e321((a12345 * a321 * 2)),
// a12345 powers: 2
// a321 powers: 2
e12345((((a12345 ^2)) + ((a321 ^2)))) )

Accurate Cube:    DualNum321(
// a12345 powers: 2
// a321 powers: 3, 1
e321((((a321 ^3)) + 3*((a12345 ^2) * a321))),
// a12345 powers: 3, 1
// a321 powers: 2
e12345((((a12345 ^3)) + 3*(a12345 * (a321 ^2)))) )

Accurate 4th pow: DualNum321(
// a12345 powers: 3, 1
// a321 powers: 3, 1
e321((4*(a12345 * (a321 ^3)) + 4*((a12345 ^3) * a321))),
// a12345 powers: 4, 2
// a321 powers: 4, 2
e12345((((a12345 ^4)) + ((a321 ^4)) + 6*((a12345 ^2) * (a321 ^2)))) )

Accurate 5th pow: DualNum321(
// a12345 powers: 4, 2
// a321 powers: 5, 3, 1
e321((((a321 ^5)) + 10*((a12345 ^2) * (a321 ^3)) + 5*((a12345 ^4) * a321))),
// a12345 powers: 5, 3, 1
// a321 powers: 4, 2
e12345((((a12345 ^5)) + 5*(a12345 * (a321 ^4)) + 10*((a12345 ^3) * (a321 ^2)))) )

Accurate 6th pow: DualNum321(
// a12345 powers: 1, 3, 5
// a321 powers: 5, 3, 1
e321((6*(a12345 * (a321 ^5)) + 20*((a12345 ^3) * (a321 ^3)) + 6*((a12345 ^5) * a321))),
// a12345 powers: 6, 4, 2
// a321 powers: 6, 4, 2
e12345((((a12345 ^6)) + ((a321 ^6)) + 15*((a12345 ^2) * (a321 ^4)) + 15*((a12345 ^4) * (a321 ^2)))) )


Base:             TripleNum( e4(a4), e5(a5), e12345(a12345) )

Accurate Square:  TripleNum(
// a12345 powers: 1
// e4 powers: 1
e4((a12345 * a4 * 2)),
// a12345 powers: 1
// e5 powers: 1
e5((a12345 * a5 * 2)),
// a12345 powers: 2
// e4 powers: 1
// e5 powers: 1
e12345((((a12345 ^2)) + 2*(a4 * a5))) )

Accurate Cube:    TripleNum(
// a12345 powers: 2
// e4 powers: 2, 1
// e5 powers: 1
e4((3*((a12345 ^2) * a4) + 2*((a4 ^2) * a5))),
// a12345 powers: 2
// e4 powers: 1
// e5 powers: 2, 1
e5((2*(a4 * (a5 ^2)) + 3*((a12345 ^2) * a5))),
// a12345 powers: 3, 1
// e4 powers: 1
// e5 powers: 1
e12345((((a12345 ^3)) + 6*(a12345 * a4 * a5))) )

Accurate 4th pow: TripleNum(
e4((4*((a12345 ^3) * a4) + 8*(a12345 * (a4 ^2) * a5))),
e5((4*((a12345 ^3) * a5) + 8*(a12345 * a4 * (a5 ^2)))),
e12345((((a12345 ^4)) + 4*((a4 ^2) * (a5 ^2)) + 12*((a12345 ^2) * a4 * a5))) )

Accurate 5th pow: TripleNum(
e4((4*((a4 ^3) * (a5 ^2)) + 5*((a12345 ^4) * a4) + 20*((a12345 ^2) * (a4 ^2) * a5))),
e5((4*((a4 ^2) * (a5 ^3)) + 5*((a12345 ^4) * a5) + 20*((a12345 ^2) * a4 * (a5 ^2)))),
e12345((((a12345 ^5)) + 20*(a12345 * (a4 ^2) * (a5 ^2)) + 20*((a12345 ^3) * a4 * a5))) )

Accurate 6th pow: TripleNum(
e4((6*((a12345 ^5) * a4) + 24*(a12345 * (a4 ^3) * (a5 ^2)) + 40*((a12345 ^3) * (a4 ^2) * a5))),
e5((6*((a12345 ^5) * a5) + 24*(a12345 * (a4 ^2) * (a5 ^3)) + 40*((a12345 ^3) * a4 * (a5 ^2)))),
e12345((((a12345 ^6)) + 8*((a4 ^3) * (a5 ^3)) + 60*((a12345 ^2) * (a4 ^2) * (a5 ^2)) + 30*((a12345 ^4) * a4 * a5))) )


Base:             QuadNum( e4(a4), e5(a5), e321(a321), e12345(a12345) )

Accurate Square:  QuadNum(
e4((a12345 * a4 * 2)),
e5((a12345 * a5 * 2)),
e321((a12345 * a321 * 2)),
e12345((((a12345 ^2)) + ((a321 ^2)) + 2*(a4 * a5))) )

Accurate Cube:    QuadNum(
e4((3*((a12345 ^2) * a4) + ((a321 ^2) * a4) + 2*((a4 ^2) * a5))),
e5((2*(a4 * (a5 ^2)) + 3*((a12345 ^2) * a5) + ((a321 ^2) * a5))),
e321((((a321 ^3)) + 3*((a12345 ^2) * a321) + 2*(a321 * a4 * a5))),
e12345((((a12345 ^3)) + 3*(a12345 * (a321 ^2)) + 6*(a12345 * a4 * a5))) )

Accurate 4th pow: QuadNum(
e4((4*((a12345 ^3) * a4) + 4*(a12345 * (a321 ^2) * a4) + 8*(a12345 * (a4 ^2) * a5))),
e5((4*((a12345 ^3) * a5) + 8*(a12345 * a4 * (a5 ^2)) + 4*(a12345 * (a321 ^2) * a5))),
e321((4*(a12345 * (a321 ^3)) + 4*((a12345 ^3) * a321) + 8*(a12345 * a321 * a4 * a5))),
e12345((((a12345 ^4)) + ((a321 ^4)) + 6*((a12345 ^2) * (a321 ^2)) + 4*((a4 ^2) * (a5 ^2)) + 12*((a12345 ^2) * a4 * a5) + 4*((a321 ^2) * a4 * a5))) )

Accurate 5th pow: QuadNum(
e4((4*((a4 ^3) * (a5 ^2)) + 5*((a12345 ^4) * a4) + ((a321 ^4) * a4) + 10*((a12345 ^2) * (a321 ^2) * a4) + 20*((a12345 ^2) * (a4 ^2) * a5) + 4*((a321 ^2) * (a4 ^2) * a5))),
e5((4*((a4 ^2) * (a5 ^3)) + 5*((a12345 ^4) * a5) + ((a321 ^4) * a5) + 20*((a12345 ^2) * a4 * (a5 ^2)) + 10*((a12345 ^2) * (a321 ^2) * a5) + 4*((a321 ^2) * a4 * (a5 ^2)))),
e321((((a321 ^5)) + 10*((a12345 ^2) * (a321 ^3)) + 5*((a12345 ^4) * a321) + 4*(a321 * (a4 ^2) * (a5 ^2)) + 4*((a321 ^3) * a4 * a5) + 20*((a12345 ^2) * a321 * a4 * a5))),
e12345((((a12345 ^5)) + 5*(a12345 * (a321 ^4)) + 10*((a12345 ^3) * (a321 ^2)) + 20*(a12345 * (a4 ^2) * (a5 ^2)) + 20*((a12345 ^3) * a4 * a5) + 20*(a12345 * (a321 ^2) * a4 * a5))) )

Accurate 6th pow: QuadNum(
e4((6*((a12345 ^5) * a4) + 24*(a12345 * (a4 ^3) * (a5 ^2)) + 6*(a12345 * (a321 ^4) * a4) + 20*((a12345 ^3) * (a321 ^2) * a4) + 40*((a12345 ^3) * (a4 ^2) * a5) + 24*(a12345 * (a321 ^2) * (a4 ^2) * a5))),
e5((6*((a12345 ^5) * a5) + 24*(a12345 * (a4 ^2) * (a5 ^3)) + 6*(a12345 * (a321 ^4) * a5) + 40*((a12345 ^3) * a4 * (a5 ^2)) + 20*((a12345 ^3) * (a321 ^2) * a5) + 24*(a12345 * (a321 ^2) * a4 * (a5 ^2)))),
e321((6*(a12345 * (a321 ^5)) + 20*((a12345 ^3) * (a321 ^3)) + 6*((a12345 ^5) * a321) + 24*(a12345 * a321 * (a4 ^2) * (a5 ^2)) + 24*(a12345 * (a321 ^3) * a4 * a5) + 40*((a12345 ^3) * a321 * a4 * a5))),

// TODO so... analyzing this... It all makes perfect sense. But I don't think I can make static expressions for it. Even though I could generate each combination of coefficients,
//   there could be any quantity of that combination showing up, if the power is high enough. I just can't imagine a simpler or more direct way to specify the expression
//   (similar as done with DualNums on page 126) other than to manually compound multiplication on itself over and over, so it automatically distributes all valid combinations.
//   The problem with THAT is that we don't have loops, branching, or boolean expressions in the AST yet. And I kind of don't want them either. At least not for 1.0 release.
//   I might still be able to do something for a codegen-side Powf that accepts a raw f32 instead of a FloatExpr.
// (a12345) powers: 6
// (e321) powers: 6
// (a12345, a321) powers: (2, 4), (4, 2)
// (a4, a5) powers: (3, 3)
// (a12345, a4, a5) powers: (2, 2, 2), (4, 1, 1)
// (a321, a4, a5) powers: (2, 2, 2), (4, 1, 1)
// (a12345, a321, a4, a5) powers (2, 2, 1, 1)
e12345((((a12345 ^6)) + ((a321 ^6)) + 15*((a12345 ^2) * (a321 ^4)) + 8*((a4 ^3) * (a5 ^3)) + 15*((a12345 ^4) * (a321 ^2)) + 60*((a12345 ^2) * (a4 ^2) * (a5 ^2)) + 12*((a321 ^2) * (a4 ^2) * (a5 ^2)) + 30*((a12345 ^4) * a4 * a5) + 6*((a321 ^4) * a4 * a5) + 60*((a12345 ^2) * (a321 ^2) * a4 * a5))) )


*/

trait_impl_1_type_2_arg_f32!(AntiPowfImpl(builder, slf, exp) -> MultiVector {
    // TODO after you finish fixing this, fix Powf, Powi, and AntiPowi also
    let exp: FloatExpr = exp.into();
    let mut dyn_mv = DynamicMultiVector::zero();

    let mut allowed_elements: Vec<BasisElement> = slf.elements_flat().map(|it| it.1).collect();
    let qty_elements = allowed_elements.len();
    if qty_elements > 8 {
        // We can't go too crazy.
        // For context, familiar 3D CGA using 5 dimensions has a VersorOdd and VersorEven each
        // with 16 elements, but the full MultiVector has 32 elements.
        // The significance of the number `8` or `16` or `32` here is that we have to get
        // every combination of coefficients. That is, while usually BasisElements don't
        // end up paired together because they cancel, the coefficients beside each BasisElement
        // do (in these Pow functions) because there are different ways to converge to an
        // (Anti)Scalar at different powers. So if a DualNum has 2 elements, then we might be
        // concerned about 2^2=4 combinations of FloatExpr. If a QuadNum has 4 elements, then we
        // might be concerned about 2^4=16 combinations of FloatExpr. A multivector type with
        // 16 different BasisElements will require 2^16 which is roughly 65,536 combinations of
        // FloatExpr. Anything higher is too crazy, with 2^24 in the millions and 2^32 in the
        // billions. Keep in mind all this is supposed to end up printed to a source file.
        // For a 16 element multivector, each BasisElement in the result of the Powf could
        // be the sum of 65k products. 65k * 16 elements = 1 million product terms in lots of
        // sums, where each product is several many bytes. You know what.... screw it. We are
        // limiting it to 8 element multivectors. 8 * 2^8 = 2048 product terms in a multivector
        // construction. Nobody asked for powf on anything beyond DualNum anyway (except me),
        // so don't lose sleep over it.
        return None;
    }
    // allowed_elements.sort();
    // let allowed_elements = allowed_elements;
    // if allowed_elements.binary_search(&builder.ga.anti_scalar()).is_err() {
    //     return None;
    // }
    //
    // let mut elements_and_stuff: Vec<_> = slf.elements_flat().collect();
    // elements_and_stuff.sort_by_key(|a| a.1)
    // let elements_and_stuff: Vec<_> = elements_and_stuff.into_iter().enumerate()
    //     .map(|(idx, (expr, el))| {
    //         let bits = 1u8 << idx;
    //         (el, bits, expr)
    //     }).collect();
    //
    // for product_term_idx in 1u8..(1u8 << qty_elements) {
    //     let qty_factors = product_term_idx.count_ones();
    //     let mut factors_here: Vec<(FloatExpr, f32)> = vec![];
    //     //
    // }


    let r = slf.clone();
    let anti_scalar = builder.ga.anti_scalar();
    for (a, a_el) in slf.elements_flat() {
        for (b, b_el) in r.elements_flat() {
            let sop = builder.ga.anti_product(a_el, b_el);
            for p in sop.sum {
                if allowed_elements.binary_search(&p.element).is_err() {
                    return None;
                }

                let f = if a_el == b_el {
                    let a_exp = FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp.clone())), 1.0);
                    let b_exp = FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp.clone())), 1.0);
                    FloatExpr::Product(vec![(a_exp, 0.5), (b_exp, 0.5)], p.coefficient)
                } else {
                    let mut f = exp.clone() * p.coefficient * 0.5;
                    let exp_minus_one = FloatExpr::Sum(vec![(exp.clone(), 1.0)], -1.0);
                    if a_el == anti_scalar {
                        f.mul_assign(FloatExpr::Exp(Box::new(a.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                    } else {
                        f.mul_assign(a.clone());
                    }
                    if b_el == anti_scalar {
                        f.mul_assign(FloatExpr::Exp(Box::new(b.clone()), Some(Box::new(exp_minus_one.clone())), 1.0));
                    } else {
                        f.mul_assign(b.clone());
                    }
                    f
                };
                dyn_mv += (f, p.element);
            }
        }
    }
    let result = dyn_mv.construct(&builder)?;
    if result.mv_class != slf.expr_type {
        return None
    }
    builder.return_expr(result)
});

trait_impl_1_type_2_arg_i32!(AntiPowiImpl(builder, slf, exp) -> MultiVector {
    let exp = FloatExpr::FromInt(exp.into());
    let result = AntiPowf.inline(&builder, slf, exp).await?;
    builder.return_expr(result)
});