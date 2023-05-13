pub mod markers {
    pub const irep: [u8; 4] = [b'I', b'R', b'E', b'P'];
    pub const end: [u8; 4] = [b'E', b'N', b'D', 0];
}

#[derive(Debug)]
pub struct BinaryHeader {
    pub ident: [u8; 4],
    pub major_version: [u8; 2],
    pub minor_version: [u8; 2],
    pub size: i32,
    pub compiler_name: [u8; 4],
    pub compiler_version: [u8; 4],
}

#[derive(Debug)]
pub struct Section {
    pub header: SectionHeader,
    pub body_irep: Option<Box<Irep>>,
}

#[derive(Debug)]
pub struct SectionHeader {
    pub ident: [u8; 4],
    pub size: i32,

    pub rite_version: Option<[u8; 4]>,
}

#[derive(Debug, Clone)]
pub struct Irep {
    pub size: i32,
    pub nlocals: u16,
    pub nregs: u16,
    /* number of child irep, historically called rlen */
    pub rlen: u16,
    /* number of catch handler */
    pub clen: u16,
    pub ilen: u32,

    pub iseq_bin: Vec<u8>,
    pub pool: Vec<PoolItem>,
    pub syms: Vec<Sym>,

    pub children: Vec<Irep>,
}

#[derive(Debug, Clone)]
pub enum PoolItem {
    Str(Vec<u8>),
    Int32(i32),
    SStr(&'static str),
    Int64(i64),
    Float(f64),
    BigInt(),
}

#[derive(Debug, Clone)]
pub struct Sym {
    pub name: Vec<u8>,
}
