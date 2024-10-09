use axum::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let v = vec!(1, 2, 3);
  let s = &v[..];
  Ok(())
}