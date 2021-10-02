use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

pub fn str2bin(str1: &str) -> Vec<u8> {
    // 从字符串得到二进制
    let mut bin1 = Vec::with_capacity(str1.len() * 8);
    for &byte in str1.as_bytes() {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            bin1.push(bit as u8);
        }
    }
    return bin1;
}

pub fn bin2str(bin1: &[u8]) -> Result<String, &'static str> {
    if bin1.len() % 8 != 0 {
        return Err("len of input should be 8n");
    }

    let bytes: Vec<u8> = bin1
        .chunks(8)
        .map(|chunk| {
            chunk.iter().enumerate().fold(0, |acc, (i, &bit)| {
                // 确保bit要么是0，要么是1
                if bit > 1 {
                    return acc;
                }
                // 将8个比特组合成一个字节
                acc | (bit << (7 - i))
            })
        })
        .collect();

    // 使用 String::from_utf8_lossy 来处理可能的无效UTF-8序列
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}



pub struct UtilWithCrypto {
    seed: [u8; 32],
}

impl UtilWithCrypto {
    pub fn new(pwd: &str) -> Self {
        let mut seed: [u8; 32] = [0u8; 32];
        let bytes = pwd.as_bytes();
        let end = bytes.len().min(32);
        seed[..end].copy_from_slice(&bytes[..end]);
        Self { seed }
    }

    pub fn bytes2bin(&self, bytes1: Vec<u8>) -> Vec<u8> {
        // bytes 得到 二进制
        let mut rng: StdRng = SeedableRng::from_seed(self.seed);

        let mut bin1 = Vec::with_capacity(bytes1.len() * 8);
        for byte in bytes1 {
            let rand_num: u8 = rng.gen_range(0..=255);
            let byte1 = byte ^ rand_num;
            for i in (0..8).rev() {
                let bit = (byte1 >> i) & 1;
                bin1.push(bit as u8);
            }
        }
        return bin1;
    }


    pub fn bin2bytes(&self, bin1: Vec<u8>) -> Vec<u8> {
        let mut bin1 = bin1.clone();
        // 补充零以使长度成为 8 的倍数
        while bin1.len() % 8 != 0 {
            bin1.push(0);
        }


        let mut rng: StdRng = SeedableRng::from_seed(self.seed);
        let result_bytes: Vec<u8> = bin1
            .chunks(8)
            .map(|chunk| {
                chunk.iter().enumerate().fold(0, |acc, (i, &bit)| {
                    // 确保 bit 要么是 0，要么是 1
                    if bit > 1 {
                        return acc;
                    }
                    // 将 8 个比特组合成一个字节
                    acc | (bit << (7 - i))
                })
            })
            .map(|x| {
                let rand_num: u8 = rng.gen_range(0..=255);
                x ^ rand_num
            })
            .collect();

        result_bytes
    }
}



