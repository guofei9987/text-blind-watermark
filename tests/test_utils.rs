use text_blind_watermark::{bin2str, str2bin, UtilWithCrypto};

#[test]
fn tst1() {
    // 测试转化
    let wm = "这是一段水印. This is watermark";

    let wm_bin = str2bin(wm);
    println!("Original text: {}\n", wm);
    println!("Text in binary: {:?}\n", wm_bin);
    println!("Text from binary: {}\n", bin2str(&wm_bin).unwrap());
}

#[test]
fn tst2() {
    // 测试加密
    let pwd = "这是密码";
    let text = "这是一段文字";

    let util_with_crypto = UtilWithCrypto::new(pwd);
    let text_bin = util_with_crypto.bytes2bin(text.as_bytes().to_vec());
    println!("original text: {}\n", text);
    println!("text in binary: {:?}\n", text_bin);

    let text2 = util_with_crypto.bin2bytes(text_bin);
    println!("text from binary: {:?}\n", String::from_utf8_lossy(text2.as_slice()));
}