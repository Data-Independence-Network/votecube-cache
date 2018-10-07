
pub struct ByteCounts {
    pub current_bits: usize,
    pub current_byte: u8,
    pub data: Vec<u8>,
}

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

    pub fn add4(mut self) {
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
        }
    }

    pub fn add3(mut self) {
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
        }
    }

    pub fn add2(mut self) {
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
        }
    }

    pub fn add1(mut self) {
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
        }
    }

    pub fn append(
        self,
        mut response: Vec<u8>
    ) {
        self.append_data(response);
        // NOTE: max page size is assumed to fin into u16
        response.extend_from_slice(&numEntriesInListBytes[6..7]);
    }

    #[inline]
    pub fn append_data(
        self,
        mut response: Vec<u8>
    ) {
        let mut num_byte_counts_bytes = self.data.len();
        response.extend(self.data);
        if self.current_bits != 0 {
            response.push(this.currentBits);
            num_byte_counts_bytes += 1;
        }

        let num_entries_in_list_bytes: [u8; 8] = unsafe {
            std::mem::transmute(*num_byte_counts_bytes);
        };
    }
}