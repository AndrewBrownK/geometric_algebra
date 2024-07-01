use crate::ga;


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
pub fn cga3d_script() {

    let cga3d = ga!(
        1 => e1, e2, e3, e_plus;
        -1 => e_minus;
        substituting_with
        e4 => 0.5 * (e_minus - e_plus);
        e5 => e_plus + e_minus;
    );

    // TODO get TraitImplRegistry, TraitDefRegistry
    // TODO MultiVectorRegistry?


}