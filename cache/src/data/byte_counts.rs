use std::mem::transmute;

pub struct ByteCounts {
    pub current_bits: u8,
    pub current_byte: u8,
    pub data: Vec<u8>,
}

/*
Counts of bytes in the actual data, supports entries from 1 to 4 bytes.
Used to determine exactly how the actual data bytes where structured, in
a serialized stream (where different data can have different byte counts).
*/
impl ByteCounts {

    pub fn new(
        capacity: usize
    ) -> ByteCounts {
        ByteCounts {
            data:  Vec::with_capacity(capacity),
            current_bits: 0,
            current_byte: 0,
        }
    }

    pub fn add4(&mut self) {
        // don't add a value, to save on computation in highest data volume cases
        // also because value 4 is usually represented with a higher bit (which
        // is absent here)
        match self.current_bits {
            0 => {
                self.current_bits = 1;
            },
            1 => {
                self.current_bits = 2;
            },
            2 => {
                self.current_bits = 3;
            },
            3 => {
                self.data.push(self.current_byte);
                self.current_byte = 0;
                self.current_bits = 0;
            }
            _ => {
                panic!("Unexpected current bits {}", self.current_bits)
            }
        }
    }

    pub fn add3(&mut self) {
        match self.current_bits {
            0 => {
                self.current_byte &= 0b11000000;
                self.current_bits = 1;
            },
            1 => {
                self.current_byte &= 0b00110000;
                self.current_bits = 2;
            },
            2 => {
                self.current_byte &= 0b00001100;
                self.current_bits = 3;
            },
            3 => {
                self.current_byte &= 0b00000011;
                self.data.push(self.current_byte);
                self.current_byte = 0;
                self.current_bits = 0;
            }
            _ => {
                panic!("Unexpected current bits {}", self.current_bits)
            }
        }
    }

    pub fn add2(&mut self) {
        match self.current_bits {
            0 => {
                self.current_byte &= 0b10000000;
                self.current_bits = 1;
            },
            1 => {
                self.current_byte &= 0b00100000;
                self.current_bits = 2;
            },
            2 => {
                self.current_byte &= 0b00001000;
                self.current_bits = 3;
            },
            3 => {
                self.current_byte &= 0b00000010;
                self.data.push(self.current_byte);
                self.current_byte = 0;
                self.current_bits = 0;
            }
            _ => {
                panic!("Unexpected current bits {}", self.current_bits)
            }
        }
    }

    pub fn add1(&mut self) {
        match self.current_bits {
            0 => {
                self.current_byte &= 0b01000000;
                self.current_bits = 1;
            },
            1 => {
                self.current_byte &= 0b00010000;
                self.current_bits = 2;
            },
            2 => {
                self.current_byte &= 0b00000100;
                self.current_bits = 3;
            },
            3 => {
                self.current_byte &= 0b00000001;
                self.data.push(self.current_byte);
                self.current_byte = 0;
                self.current_bits = 0;
            }
            _ => {
                panic!("Unexpected current bits {}", self.current_bits)
            }
        }
    }

    pub fn append(
        self,
        response: &mut Vec<u8>
    ) {
        let num_entries_in_list_bytes = self.append_data(response);
        // NOTE: max page size is assumed to fin into u16
        response.extend_from_slice(&num_entries_in_list_bytes[6..7]);
    }

/*
    Append the byte counts and return the size of the byte counts
    (which itself is broken into a byte array).
*/
    #[inline]
    pub fn append_data(
        self,
        response: &mut Vec<u8>
    ) -> [u8; 8] {
        let mut num_byte_counts_bytes: u64 = self.data.len() as u64;
        
        response.extend(self.data);
        if self.current_bits != 0 {
            response.push(self.current_bits);
            num_byte_counts_bytes += 1;
        }

        let num_entries_in_list_bytes: [u8; 8] = unsafe {
            transmute(num_byte_counts_bytes.to_be())
        };

        num_entries_in_list_bytes
    }
}