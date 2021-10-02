use text_blind_watermark::TextBlindWM;


#[test]
fn test1() {
    let pwd = "这是一段密码";
    let wm = "不可见的暗水印";
    let text = "这是一段文本，之后这段文本将会被嵌入不可见盲水印";

    let text_blind_watermark = TextBlindWM::new(pwd);

    let wm_dark = text_blind_watermark.get_wm(wm);

    print!("{}", wm_dark)
}


#[test]
fn test2() {
    let pwd = "这是一段密码";
    let wm = "不可见的暗水印";
    let text = "这是一段文本，之后这段文本将会被嵌入不可见盲水印";

    let text_blind_watermark = TextBlindWM::new(pwd);

    // 嵌入
    let text_with_wm = text_blind_watermark.embed(text, wm);
    print!("嵌入文本后：{}\n", text_with_wm);

    // 提取

    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());

    print!("提取的文本：{}\n", String::from_utf8_lossy(wm_extract.as_slice()))
}