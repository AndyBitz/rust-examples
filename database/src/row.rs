use std::convert::TryInto;

use crate::page;

#[derive(Debug)]
pub struct Row {
    pub id: [u8; 0xff],
    pub username: [u8; 0xff],
    pub age: i32,
}

impl Row {
    pub fn new(id: String, username: String, age: i32) -> Self {
        let mut row = Row {
            id: [0x0; 0xff],
            username: [0x0; 0xff],
            age
        };
        
        for (i, byte) in id.chars().enumerate() {            
            row.id[i] = byte as u8;
        }

        for (i, byte) in username.chars().enumerate() {            
            row.username[i] = byte as u8;
        }

        return row;
    }

    pub fn from_bytes(bytes: &[u8; std::mem::size_of::<Row>()]) -> Self {
        let mut id = [0x0; 0xff];
        id.copy_from_slice(&bytes[0..255]);

        let mut username = [0x0; 0xff];
        username.copy_from_slice(&bytes[255..510]);

        let mut age = [0x0; 0x4];
        age.copy_from_slice(&bytes[511..515]);

        let row = Row {
            id,
            username,
            age: i32::from_le_bytes(age),
        };

        return row;
    }

    pub fn serialize(&self) -> [u8; std::mem::size_of::<Row>()] {
        let mut index: usize = 0;
        let mut output = [0x0; std::mem::size_of::<Row>()];

        for byte in self.id {
            output[index] = byte;
            index += 1;
        }

        for byte in self.username {
            output[index] = byte;
            index += 1;
        }

        for byte in self.age.to_le_bytes() {
            output[index] = byte;
            index += 1;
        }
        
        return output;
    }
}

pub struct RowReader<'a> {
    page: &'a page::Page,
    row_size: usize,
    current: usize,
    last: usize,
}

impl RowReader<'_> {
    pub fn new<'a>(page: &'a page::Page, row_size: usize) -> RowReader<'a> {
        RowReader {
            page,
            row_size,
            current: 0,
            last: (page.data_length as usize / row_size) - 1 
        }
    }
}

impl Iterator for RowReader<'_> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.last {
            return None;
        }

        let start = self.current * self.row_size;
        let end = start + self.row_size;
        self.current += 1;

        Some(Row::from_bytes(&self.page.data[start..end].try_into().unwrap()))
    }
}
