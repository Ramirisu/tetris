use rand::Rng;

// The seed of the rng is 32 bytes long. But only the lower 8 bytes are used for seeding due to the UI's limitation.
pub const SEED_BYTE_COUNT: usize = 8;
pub const SEED_HEX_COUNT: usize = SEED_BYTE_COUNT * 2;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Seed {
    pub bytes: [u8; SEED_BYTE_COUNT],
}

impl Seed {
    pub fn new() -> Self {
        Self {
            bytes: rand::rng().random(),
        }
    }

    pub fn increment(&mut self, hex_index: usize) {
        let idx = hex_index / 2;
        if (hex_index % 2) > 0 {
            self.bytes[idx] =
                (self.bytes[idx] & 0x0F) | (self.bytes[idx].wrapping_add(0x10) & 0xF0);
        } else {
            self.bytes[idx] =
                (self.bytes[idx].wrapping_add(0x01) & 0x0F) | (self.bytes[idx] & 0xF0);
        }
    }

    pub fn decrement(&mut self, hex_index: usize) {
        let idx = hex_index / 2;
        if (hex_index % 2) > 0 {
            self.bytes[idx] =
                (self.bytes[idx] & 0x0F) | (self.bytes[idx].wrapping_sub(0x10) & 0xF0);
        } else {
            self.bytes[idx] =
                (self.bytes[idx].wrapping_sub(0x01) & 0x0F) | (self.bytes[idx] & 0xF0);
        }
    }
}

impl Into<[u8; 32]> for Seed {
    fn into(self) -> [u8; 32] {
        let mut bytes = [0; 32];
        bytes[0..SEED_BYTE_COUNT].copy_from_slice(&self.bytes);
        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seed_default() {
        let seed = Seed::default();
        assert!(seed.bytes.iter().all(|byte| *byte == 0));
    }
}
