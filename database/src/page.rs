#[derive(Debug)]
pub struct Page {
    /**
     * The offset of the page.
     */
    pub offset: u64,
    /**
     * The length in bytes of the data used.
     */
    pub data_length: u16,
    /**
     * The user content of this page.
     */
    pub data: [u8; 0xffff - 0x8 - 0x2]
}

impl Page {
    pub fn new(offset: u64) -> Page {
        Page {
            offset: offset,
            data_length: 0x0,
            data: [0x0; 0xffff - 0x8 - 0x2]
        }
    }

    pub fn from_buffer(buffer: &[u8; 0xffff]) -> Page {        
        let mut offset_bytes: [u8; 0x8] = [0x0; 0x8];
        offset_bytes.copy_from_slice(&buffer[0x0..0x8]);

        let mut data_length_bytes: [u8; 0x2] = [0x0; 0x2];
        data_length_bytes.copy_from_slice(&buffer[0x8..0xa]);

        let mut data_bytes: [u8; 0xffff - 0x8 - 0x2] = [0x0; 0xffff - 0x8 - 0x2];
        data_bytes.copy_from_slice(&buffer[0xa..0xffff]);

        let page = Page {
            offset: u64::from_le_bytes(offset_bytes),
            data_length: u16::from_le_bytes(data_length_bytes),
            data: data_bytes,
        };

        return page;
    }

    // TODO: A more memory efficient way would be to convert the type somehow
    // and then consume the reference.
    pub fn serialize(&self) -> [u8; 0xffff] {
        let mut index: usize = 0;
        let mut output = [0x0; 0xffff];

        for byte in self.offset.to_le_bytes() {
            output[index] = byte;
            index += 1;
        }

        for byte in self.data_length.to_le_bytes() {
            output[index] = byte;
            index += 1;
        }

        for byte in self.data {
            output[index] = byte;
            index += 1;
        }

        output
    }

    /**
     *
     */
    pub fn put(&mut self, data: &[u8]) -> i32 {
        let mut bytes_written = 0;

        for byte in data {
            // TODO: Overflow error handling
            self.data_length += 1;
            bytes_written += 1;
            self.data[(self.data_length) as usize] = *byte;
        }

        return bytes_written;
    }
}
