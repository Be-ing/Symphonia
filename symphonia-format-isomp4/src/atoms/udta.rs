// Symphonia
// Copyright (c) 2019-2021 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBytes;
use symphonia_core::meta::MetadataLog;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, MetaAtom};

/// User data atom.
#[derive(Debug)]
pub struct UdtaAtom {
    /// Atom header.
    header: AtomHeader,
    /// Metadata atom.
    pub meta: Option<MetaAtom>,
}

impl UdtaAtom {
    /// Consume any metadata, and push it onto provided `MetadataLog`.
    pub fn take_metadata(&mut self, log: &mut MetadataLog) {
        if let Some(meta) = self.meta.take() {
            meta.take_metadata(log)
        }
    }
}

impl Atom for UdtaAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    #[allow(clippy::single_match)]
    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);
        
        let mut meta = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::Meta => {
                    meta = Some(iter.read_atom::<MetaAtom>()?);
                }
                _ => ()
            }
        }

        Ok(UdtaAtom {
            header,
            meta,
        })
    }
}