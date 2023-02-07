pub struct ReedSolomon {
    data: Vec<u64>,
    parity: Vec<u64>,
    size: u32,
}

impl Default for ReedSolomon {
    fn default() -> ReedSolomon {
        ReedSolomon {
            data: Vec::with_capacity(8),
            parity: Vec::with_capacity(16),
            size: 8,
        }
    }
}

impl ReedSolomon {
    pub fn new(size: u64) -> ReedSolomon {
        ReedSolomon {
            data: Vec::with_capacity(size),
            parity: Vec::with_capacity(size*2),
            size: size,
        }
    }

    pub fn set_data(&mut self, &data: Vec<u64>) -> &Vec<u64> {
        
    }
}

