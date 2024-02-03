use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};


pub struct BytesBinConverter {
    seed: [u8; 32],
    byte_map_bin: [[u8; 8]; 256],
}

impl BytesBinConverter {
    pub fn new() -> Self {
        Self::new_with_pwd("")
    }

    pub fn new_with_pwd(pwd: &str) -> Self {
        let mut seed: [u8; 32] = [0u8; 32];
        let bytes = pwd.as_bytes();
        let end = bytes.len().min(32);
        seed[..end].copy_from_slice(&bytes[..end]);

        let mut byte_map_bin = [[0; 8]; 256];
        for byte1 in 0..256 {
            for idx in 0..8 {
                byte_map_bin[byte1][idx] = ((byte1 >> (7 - idx)) & 1) as u8;
            }
        }
        Self { seed, byte_map_bin }
    }


    pub fn bytes2bin(&self, bytes1: &Vec<u8>) -> Vec<u8> {
        return bytes1.iter().flat_map(
            |byte1| self.byte_map_bin[*byte1 as usize]).collect();
    }

    pub fn bin2bytes(&self, bin1: &Vec<u8>) -> Vec<u8> {
        return bin1.chunks(8).map(|chunk| {
            chunk.iter().enumerate().fold(0, |acc, (i, &bit)| {
                acc | (bit << (7 - i))
            })
        }).collect();
    }

    pub fn encrypt_bytes2bin(&self, bytes1: &Vec<u8>) -> Vec<u8> {
        let mut rng: StdRng = SeedableRng::from_seed(self.seed);
        self.bytes2bin(&bytes1.iter()
            .map(|byte1| byte1 ^ rng.gen_range(0..=255)).collect::<Vec<u8>>()
        )
    }


    pub fn bin2bytes_decrypt(&self, bin1: &Vec<u8>) -> Vec<u8> {
        let mut rng: StdRng = SeedableRng::from_seed(self.seed);
        self.bin2bytes(&bin1).iter().map(|x| x ^ rng.gen_range(0..=255)).collect()
    }
}


