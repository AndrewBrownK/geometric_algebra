use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    generate_multi_bases();
}


/// Code-gen for code-gen for code-gen.... kind of absurd
/// Speaking of absurd
/// What if someone wants 16 different basis elements?
/// Well I thought it would be sooo coooool to make basis element bitflags
/// In an 8 dimensional algebra, there are 256 elements (including scalar)
/// But in a 16 dimensional algebra, there are 65,534 elements
/// So while pre-generating combined bases is certainly cool,
/// maybe it is dubious to call 65k constants "convenient".
/// So yet again, this is another great situation for feature flags. But we can't just
/// put the flag on 65k constants, that's still a lot to digest.
/// So instead, we'll make sure the feature completely changes the file, so it is actually
/// smaller and easier to compile, and only gets chunky if you are insane and want to do
/// 9+ dimensional stuff
fn generate_multi_bases() {
    let out_dir = &env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(out_dir).join("generated_elements.rs");
    let mut f = File::create(dest_path).unwrap();

    // 8 dimensions -> 255 elements
    let mut max = u8::MAX as u16;
    if cfg!(feature = "large-basis-elements") {
        // 12 dimensions -> 4095 elements
        max = (1u16 << 12) - 1;
    }
    if cfg!(feature = "very-large-basis-elements") {
        // 16 dimensions -> 65k elements
        max = u16::MAX;
    }
    let mut numbers: Vec<u16> = (0..=max).collect();
    if !cfg!(feature = "large-basis-elements") {
        numbers.push(1u16 << 8);
        numbers.push(1u16 << 9);
        numbers.push(1u16 << 10);
        numbers.push(1u16 << 11);
    }
    if !cfg!(feature = "very-large-basis-elements") {
        numbers.push(1u16 << 12);
        numbers.push(1u16 << 13);
        numbers.push(1u16 << 14);
        numbers.push(1u16 << 15);
    }
    numbers.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()).then_with(|| b.reverse_bits().cmp(&a.reverse_bits())));

    write!(
        f,
        "const fn element(signature: BasisSignature) -> BasisElement {{
    BasisElement {{
        coefficient: 1,
        signature,
        display_name: ConstOption::None,
    }}
}}
"
    )
    .unwrap();

    for num in numbers {
        let mut combined_basis = if num == 0 { "scalar".to_string() } else { "e".to_string() };
        for i in 0..16 {
            if num & (1 << i) != 0 {
                combined_basis.push(char::from_digit(i, 16).unwrap().to_ascii_uppercase());
            }
        }

        let line = format!("pub const {combined_basis}: BasisElement = element(BasisSignature::from_bits_retain(0b{num:016b}));\n");
        f.write(line.as_bytes()).unwrap();
    }
}
