use std::io::{Bytes, Write};
use crate::utils::UtilWithCrypto;


pub struct TextBlindWM {
    util_with_crypto: UtilWithCrypto,
    chr0: char,
    chr1: char,
}

impl TextBlindWM {
    pub fn new(pwd: &str) -> Self {
        Self {
            util_with_crypto: UtilWithCrypto::new(pwd),
            chr0: char::from_u32(0x200c).unwrap(),
            chr1: char::from_u32(0x200d).unwrap(),
        }
    }

    pub fn set_algo_type(&mut self, chr: (usize, usize)) {
        let all_chr_wm = [
            char::from_u32(0x1d).unwrap()
            , char::from_u32(0x7f).unwrap()
            , char::from_u32(0x200b).unwrap()
            , char::from_u32(0x200c).unwrap()
            , char::from_u32(0x200d).unwrap()
        ];

        self.chr0 = all_chr_wm[chr.0];
        self.chr1 = all_chr_wm[chr.1];
    }


    pub fn get_wm(&self, wm: &str) -> String {
        let wm_bin = self.util_with_crypto.bytes2bin(wm.as_bytes().to_vec());

        let wm_dark: String = wm_bin.into_iter()
            .map(|bit| {
                if bit == 0 { return self.chr0; } else {
                    return self.chr1;
                }
            })
            .collect();
        return wm_dark;
    }

    pub fn embed(&self, text: &str, wm: &str) -> String {
        let wm_dark = self.get_wm(wm);
        // TODO:这里之后做成随机嵌入
        return format!("{}{}", text, wm_dark);
    }

    pub fn extract(&self, text_with_wm: &str) -> Vec<u8> {
        let mut idx_left: Option<usize> = None;
        let mut idx_right: Option<usize> = None;

        for (idx, chr) in text_with_wm.chars().enumerate() {
            if chr == self.chr0 || chr == self.chr1 {
                if idx_left.is_none() {
                    idx_left = Some(idx);
                }
            } else {
                if idx_left.is_some() && idx_right.is_none() {
                    idx_right = Some(idx);
                    break;
                }
            }
        }


        if idx_left.is_some() {
            if idx_right.is_none() {
                idx_right = Some(text_with_wm.len());
            }
        } else {
            print!("There is no watermark");
        }


        let wm_bin: Vec<u8> = text_with_wm.chars()
            .skip(idx_left.unwrap()).take(idx_right.unwrap() - idx_left.unwrap())
            .map(|x| { if x == self.chr0 { 0u8 } else { 1u8 } })
            .collect();


        return self.util_with_crypto.bin2bytes(wm_bin);
    }
}

