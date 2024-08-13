// Conformal uwu 3d

use crate::ga;

#[allow(non_upper_case_globals)]
mod elements {
    use crate::algebra2::basis::elements::*;
    use crate::algebra2::basis::BasisElement;

    pub const sugar: BasisElement = e1.with_name("sugar", false);
    pub const spice: BasisElement = e2.with_name("spice", false);
    pub const everything_nice: BasisElement = e3.with_name("everything_nice", false);
    pub const hungry: BasisElement = eP.with_name("hungry", false);
    pub const horny: BasisElement = eM.with_name("horny", false);
    pub const lazy: BasisElement = e4.with_name("lazy", false);
    pub const desperate: BasisElement = e5.with_name("desperate", false);
}
#[allow(non_upper_case_globals)]
mod generators {
    use crate::algebra2::basis::generators::GeneratorElement;
    use crate::algebra2::basis::generators::*;

    pub const girls: i8 = 1;
    pub const boys: i8 = -1;

    pub const sugar: GeneratorElement = e1;
    pub const spice: GeneratorElement = e2;
    pub const everything_nice: GeneratorElement = e3;
    pub const hungry: GeneratorElement = eP;
    pub const horny: GeneratorElement = eM;
    pub const lazy: GeneratorElement = e4;
    pub const desperate: GeneratorElement = e5;
}

pub fn uwu_script() {
    use generators::*;

    // let uwu = ga!(
    //     girls => sugar, spice, everything_nice, hungry;
    //     boys => horny;
    //     where
    //     lazy => 0.5 * (horny - hungry);
    //     desperate => horny + hungry;
    // );
}
