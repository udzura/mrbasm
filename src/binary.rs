use bytes::BufMut;

use crate::rite;
use std::io;

#[derive(Debug)]
pub struct Binary {
    pub binary_header: rite::BinaryHeader,
    pub sections: Vec<rite::Section>,
}

impl Binary {
    pub fn new() -> Self {
        let binary_header = rite::BinaryHeader {
            ident: rite::IDENT,
            major_version: [b'0', b'3'],
            minor_version: [b'0', b'0'],
            size: -1, // marker
            compiler_name: crate::COMPILER_NAME,
            compiler_version: [b'0', b'0', b'0', b'0'],
        };
        let irep_section = rite::Section {
            header: rite::SectionHeader {
                ident: rite::markers::IREP,
                size: -1,

                rite_version: Some([b'0', b'3', b'0', b'0']),
            },
            body_irep: None,
        };
        let end_secsion = rite::Section {
            header: rite::SectionHeader {
                ident: rite::markers::END,
                size: 8,

                rite_version: None,
            },
            body_irep: None,
        };
        let sections = vec![irep_section, end_secsion];

        Self {
            binary_header,
            sections,
        }
    }

    pub fn write(&self, mut out: impl io::Write) -> Result<(), io::Error> {
        let b = &self.binary_header;
        let mut buf = bytes::BytesMut::new();
        buf.put_slice(&b.ident);
        buf.put_slice(&b.major_version);
        buf.put_slice(&b.minor_version);
        let size_pos = buf.len();
        buf.put_i32(-1);
        buf.put_slice(&b.compiler_name);
        buf.put_slice(&b.compiler_version);

        for section in self.sections.iter() {
            match section.header.ident {
                rite::markers::IREP => {
                    let h = &section.header;
                    let i = section.body_irep.as_ref().unwrap();

                    buf.put_slice(&h.ident);
                    buf.put_i32(12 + i.size);
                    buf.put_slice(&h.rite_version.unwrap());

                    buf.put_i32(i.size);
                    buf.put_u16(i.nlocals);
                    buf.put_u16(i.nregs);
                    buf.put_u16(i.rlen);
                    buf.put_u16(i.clen);
                    buf.put_u32(i.ilen);
                    buf.put_slice(&i.iseq_bin);

                    buf.put_u16(i.pool.len() as u16);
                    for pi in i.pool.iter() {
                        match pi {
                            rite::PoolItem::Str(s) => {
                                buf.put_u8(0); // IREP_TT_STR
                                buf.put_u16(s.len() as u16);
                                buf.put_slice(s);
                                buf.put_u8(0);
                            }
                            _ => {
                                todo!("unsupported")
                            }
                        }
                    }

                    buf.put_u16(i.syms.len() as u16);
                    for s in i.syms.iter() {
                        buf.put_u16(s.name.len() as u16);
                        buf.put_slice(&s.name);
                        buf.put_u8(0);
                    }

                    for _ in i.children.iter() {
                        eprintln!("WIP children not supported")
                    }
                }
                rite::markers::END => {
                    let h = &section.header;
                    buf.put_slice(&h.ident);
                    buf.put_i32(h.size);
                }
                _ => {
                    unreachable!("unknown section ident")
                }
            }
        }

        // finally write back total binsize
        let size = buf.len();
        buf[size_pos] = ((size >> 24) & 0xff) as u8;
        buf[size_pos + 1] = ((size >> 16) & 0xff) as u8;
        buf[size_pos + 2] = ((size >> 8) & 0xff) as u8;
        buf[size_pos + 3] = (size & 0xff) as u8;

        let freeze = buf.freeze();
        out.write(&freeze[..])?;
        out.flush()?;

        Ok(())
    }
}
