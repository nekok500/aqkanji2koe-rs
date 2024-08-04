# aqkanji2koe-rs
## 使い方
```rust
let kanji2koe = AqKanji2Koe::create("./aq_dic")?
let koe = kanji2koe.convert("ゆっくりしていってね！")?;

assert_eq!(koe, "ユック'リ/_シテイッテ'ヌ、") // 評価版はヌになる
```

利用する際は[アクエスト社のサイト](https://www.a-quest.com/licence.html)を確認し、適切なライセンスを設定してください。
