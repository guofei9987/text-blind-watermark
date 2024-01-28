use rand::Rng;
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

    pub fn new_with_char(pwd: &str, chr0: u32, chr1: u32) -> Self {
        // User defined chars. see ./chars.md
        Self {
            util_with_crypto: UtilWithCrypto::new(pwd),
            chr0: char::from_u32(chr0).unwrap(),
            chr1: char::from_u32(chr1).unwrap(),
        }
    }

    pub fn generate_watermark(&self, wm: &Vec<u8>) -> String {
        let wm_bin = self.util_with_crypto.bytes2bin(wm);

        let wm_dark: String = wm_bin.into_iter()
            .map(|bit| {
                return if bit == 0 { self.chr0 } else { self.chr1 };
            })
            .collect();
        return wm_dark;
    }

    pub fn embed(&self, text: &str, wm: &Vec<u8>) -> String {
        let text = self.remove_watermark(text);
        let text_char: Vec<char> = text.chars().collect();
        let wm_dark = self.generate_watermark(wm);
        let mut res = String::with_capacity(text.len() + wm_dark.len());

        let mut rng = rand::thread_rng();
        let insert_idx = rng.gen_range(0..=text_char.len());

        // 前半部分
        for chr in text_char.iter().take(insert_idx) {
            res.push(*chr)
        }

        res.push_str(wm_dark.as_str());

        for chr in text_char.iter().skip(insert_idx) {
            res.push(*chr)
        }
        return res;
    }

    pub fn remove_watermark(&self, text: &str) -> String {
        let text_char: Vec<char> = text.chars().collect();
        let mut res = String::with_capacity(text.len());
        for chr in text_char {
            if chr != self.chr0 && chr != self.chr1 {
                res.push(chr);
            }
        }
        return res;
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
            return vec![];
        }


        let wm_bin: Vec<u8> = text_with_wm.chars()
            .skip(idx_left.unwrap()).take(idx_right.unwrap() - idx_left.unwrap())
            .map(|x| { if x == self.chr0 { 0u8 } else { 1u8 } })
            .collect();


        return self.util_with_crypto.bin2bytes(wm_bin);
    }
}

