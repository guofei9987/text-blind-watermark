use text_blind_watermark::BytesBinConverter;

#[test]
fn test1() {
    // 测试加密
    let text = "这是一段文字";

    let util_with_crypto = BytesBinConverter::new();
    let text_bin = util_with_crypto.bytes2bin(&text.as_bytes().to_vec());
    println!("original text: {}\n", text);
    println!("text in binary: {:?}\n", text_bin);

    let text2 = util_with_crypto.bin2bytes(&text_bin);
    println!("text from binary: {:?}\n", String::from_utf8_lossy(text2.as_slice()));
}

#[test]
fn test2() {
    let pwd = "这是密码";
    let text = "这是一段文字";

    let util_with_crypto = BytesBinConverter::new_with_pwd(pwd);
    let text_bin = util_with_crypto.encrypt_bytes2bin(&text.as_bytes().to_vec());
    println!("original text: {}\n", text);
    println!("text in binary: {:?}\n", text_bin);

    let text2 = util_with_crypto.bin2bytes_decrypt(&text_bin);
    println!("text from binary: {:?}\n", String::from_utf8_lossy(text2.as_slice()));
}