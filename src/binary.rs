use crate::rite::{self, Section};
use std::io;

#[derive(Debug)]
pub struct Binary {
    pub binary_header: rite::BinaryHeader,
    pub sections: Vec<rite::Section>,
}

impl Binary {
    pub fn new() -> Self {
        let binary_header = rite::BinaryHeader {
            ident: [b'R', b'I', b'T', b'E'],
            major_version: [b'0', b'3'],
            minor_version: [b'0', b'0'],
            size: -1, // marker
            compiler_name: crate::compiler_name,
            compiler_version: [b'0', b'0', b'0', b'0'],
        };
        let irep_section = rite::Section {
            header: rite::SectionHeader {
                ident: rite::markers::irep,
                size: -1,

                rite_version: Some([b'0', b'3', b'0', b'0']),
            },
            body_irep: None,
        };
        let end_secsion = rite::Section {
            header: rite::SectionHeader {
                ident: rite::markers::end,
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
        Ok(())
    }
}
