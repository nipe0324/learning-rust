try anyhow
---

- <https://github.com/dtolnay/anyhow>
- Rustのエラーハンドリングを簡単にするクレート
- `std::result::Result`型を使いやすくした`anyhow::Result<T>`型が便利
- `context`や`with_context`でエラーメッセージに簡単にコンテキスト情報をつけることができる
- `anyhow!`や`bail!`マクロで`anyhow::Result<T>`型のエラーを簡単に作成できる
