const fn cfg_zero_based(e: u16) -> u16 {
    return if cfg!(feature = "zero-indexed-bases") {
        e * 2
    } else {
        e
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BasisSignature: u16 {
        #[cfg(feature = "zero-indexed-bases")]
        const e0 = 0x1;
        const e1 = cfg_zero_based(0x1);
        const e2 = cfg_zero_based(0x2);
        const e3 = cfg_zero_based(0x4);
        const e4 = cfg_zero_based(0x8);
        const e5 = cfg_zero_based(0x10);
        const e6 = cfg_zero_based(0x20);
        const e7 = cfg_zero_based(0x40);
        const e8 = cfg_zero_based(0x80);
        const e9 = cfg_zero_based(0x100);
        const eA = cfg_zero_based(0x200);
        const eB = cfg_zero_based(0x400);
        const eC = cfg_zero_based(0x800);
        const eD = cfg_zero_based(0x1000);
        const eE = cfg_zero_based(0x2000);
        const eF = cfg_zero_based(0x4000);
        #[cfg(not(feature = "zero-indexed-bases"))]
        const eG = 0x8000;
    }
}


