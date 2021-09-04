use std::io::prelude::*;
use std::io::SeekFrom;

use crate::page;

pub struct PageReader {
    file: std::fs::File,
    offset: usize,
}

impl PageReader {
    pub fn new(file: std::fs::File) -> Self {
        PageReader {
            file,
            offset: 0
        }
    }

    pub fn skip_to_page(&mut self, page_offset: usize) {
        let page_start = page_offset * 0xffff;

        if page_offset == self.offset {
            return;
        }

        // If the difference is small enough, just read everything until
        // as it is more performant than seeking.
        if page_offset > self.offset {
            let pages_to_read = (page_offset - self.offset) - 1;

            // Seems to skip 10 pages at max. with current dataset.
            // Further tests are needed to check when seek would be more performant.
            // Maybe it's related to how much the file system reads ahead?
            if 11 > pages_to_read {
                for _ in 0..pages_to_read {
                    self.read_next();
                }

                return;
            }
        }

        self.file.seek(SeekFrom::Start(page_start as u64)).unwrap();
    }

    pub fn read_next(&mut self) -> Option<page::Page> {
        let mut page_buffer = [0x0u8; 0xffff];

        let done = match self.file.read_exact(&mut page_buffer) {
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => None,
            Err(e) => panic!("{}", e),
            Ok(r) => Some(r),
        };

        if done == None {
            return None;
        }

        let page = page::Page::from_buffer(&page_buffer);
        self.offset = page.offset as usize;

        return Some(page);
    }

    pub fn read_page(&mut self, page_offset: usize) -> Option<page::Page> {
        self.skip_to_page(page_offset);
        self.read_next()
    }
}

impl Iterator for PageReader {
    type Item = page::Page;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_next()
    }
}
