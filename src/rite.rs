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

#[derive(Debug)]
pub struct Irep {}
