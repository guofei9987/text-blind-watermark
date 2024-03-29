use text_blind_watermark::TextBlindWM;
use std::fs;


#[test]
fn test1() {
    let pwd = "p@ssw0rd";
    let wm = "watermark: guofei.site";
    let text = "这是一段文本，之后这段文本将会被嵌入不可见盲水印";

    let text_blind_watermark = TextBlindWM::new(pwd);

    // embed
    let text_with_wm = text_blind_watermark.embed(text, &wm.as_bytes().to_vec());
    println!("text with watermark：{}", text_with_wm);

    // extract
    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());

    println!("watermark extracted：{}", String::from_utf8_lossy(wm_extract.as_slice()))
}


#[test]
fn test_add_twice() {
    let pwd = "p@ssw0rd";
    let wm = "watermark: guofei.site";
    let text = "这是一段文本，之后这段文本将会被嵌入不可见盲水印";

    let text_blind_watermark = TextBlindWM::new(pwd);

    // embed
    let text_with_wm = text_blind_watermark.embed(text, &wm.as_bytes().to_vec());
    println!("text with watermark：{}", text_with_wm);


    // extract
    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());
    println!("watermark extracted：{}", String::from_utf8_lossy(wm_extract.as_slice()))
}


#[test]
fn test2() {
    let pwd = "这是一段密码. This is password";
    let wm = "不可见的暗水印. This is watermark";
    let ori_filename = "./files/file.txt";
    let file_with_wm = "./files/outputs/file_with_wm.txt";


    let text_blind_watermark = TextBlindWM::new(pwd);

    let text = fs::read_to_string(ori_filename).unwrap();

    // embed
    let text_with_wm = text_blind_watermark.embed(text.as_str(), &wm.as_bytes().to_vec());
    // write into file
    fs::write(file_with_wm, text_with_wm).unwrap();
    println!("text with watermark saved in file <{}>", file_with_wm);

    // read text and extract the watermark
    let text_with_wm = fs::read_to_string(file_with_wm).unwrap();

    // extract
    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());

    println!("watermark extracted：{}", String::from_utf8_lossy(wm_extract.as_slice()))
}


#[test]
fn test3() {
    let pwd = "p@ssw0rd";
    let wm = "watermark: guofei.site";
    let text = "这是一段文本，之后这段文本将会被嵌入不可见盲水印";

    let text_blind_watermark = TextBlindWM::new(pwd);

    let text_with_wm = text_blind_watermark.add_wm_at_last(text, &wm.as_bytes().to_vec());
    let text_with_wm = text_blind_watermark.embed(text_with_wm.as_str(), &wm.as_bytes().to_vec());
    // extract
    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());
    println!("watermark extracted：{}", String::from_utf8_lossy(wm_extract.as_slice()))
}