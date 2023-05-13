use std::{error::Error, fs};

use mrbasm::binary;
use mrbasm::rite::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut bin = binary::Binary::new();
    let pool = vec![PoolItem::Str("Hello".as_bytes().to_vec())];
    let syms = vec![Sym {
        name: "puts".as_bytes().to_vec(),
    }];
    let iseq_bin: Vec<u8> = [81, 2, 0, 45, 1, 0, 1, 56, 1, 105].into();

    let iseq = Irep {
        size: (4 + 2 + 2 + 2 + 2 + 4 + 10 + 2 + 1 + (2 + 5 + 1) + 2 + (2 + 4 + 1)),
        nlocals: 1,
        nregs: 4,
        rlen: 0,
        clen: 0,
        ilen: iseq_bin.len() as u32,
        iseq_bin,
        children: vec![],
        pool,
        syms,
    };

    let irep = bin.sections.get_mut(0).unwrap();
    irep.body_irep = Some(Box::new(iseq));

    let out = fs::File::create("./sample.mrb")?;

    bin.write(&out)?;
    return Ok(());
}
