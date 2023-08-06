try thiserror
---

- <https://github.com/dtolnay/thiserror>
- `thiserror`は標準ライブラリの [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)トレイトの便利なderiveマクロを提供している
- カスタムエラータイプを実装する際の`fmt::Display`、`std::error::Error`、`From<T>`トレイトの実装をほぼ省略できる
