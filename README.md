# static_files_enum

static_files_enum is a rust static files lib for axum

```sh
cargo add static_files_enum # still not on crates.io (yet)
```

# Declare your static files

This will embed the static files in your binary when you call `StaticFile::new()`.
It also looks for files in the /static folder in the root of your project.

```rust
use static_files_enum::StaticFiles;

#[derive(StaticFiles)]
enum StaticFile {
  #[path("htmx.js")]
  Htmx,
  #[path("tailwind.css")]
  Tailwind
}
```

Then tell axum to serve them:

```rust
use axum::routing::get;
use axum::{Server, Router};
use axum::response::Response;

#[tokio::main]
async fn main() {
  let static_files = StaticFile::new(); // this embeds the static files in your binary and loads the contents into memory
  let addr = "127.0.0.1:9001".parse().unwrap();
  let router = Router::new().route("/static/*file", get(|uri: Uri| -> impl IntoResponse {
    static_files.get(uri.path.to_string()) // this retrieves the static files from memory not from disk
  }))

  Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}
```

