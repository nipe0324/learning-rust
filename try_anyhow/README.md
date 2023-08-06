try anyhow
---

- rustのエラーハンドリングを簡単にする
- `std::error::Error` を実装しているエラーならなんでもこいやーとできる

```rs
// 使った場合
fn do_something() -> Result<int> {
  let i = foo()?; // FooError
  let j = bar(i)?; // BarError
  Ok(j)
}

// 使わない場合
fn do_something() -> Result<int, std::error::Error> {
  let i = foo()
}
```
