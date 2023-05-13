use std::{error::Error, fs};

use mrbasm::binary;

fn main() -> Result<(), Box<dyn Error>> {
    let bin = binary::Binary::new();
    let out = fs::File::create("./sample.mrb")?;
    bin.write(&out)?;
    return Ok(());
}
