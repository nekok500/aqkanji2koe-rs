use aqkanji2koe::AqKanji2Koe;

fn main() -> anyhow::Result<()> {
    let koe = AqKanji2Koe::create("./aq_dic")?.convert("ゆっくりしていってね！")?;
    let wav = aquestalk::synthe(&koe, 50).unwrap();
    std::fs::write("hello.wav", &wav)?;

    Ok(())
}
