# static_files_enum

static_files_enum is a rust static files lib for axum

```sh
cargo add static_files_enum # still not on crates.io (yet)
```

# Declare your static files

This will embed the static files in your binary at compile time with `include_str!`.
It will try to find the files starting from the root of your project: `CARGO_MANIFEST_DIR`.

```rust
use static_files_enum::StaticFiles;

#[derive(StaticFiles)]
enum StaticFile {
  #[file("/htmx.js")]
  Htmx,
  #[file("/tailwind.css")]
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
  // The new function calls `include_str!` and stores the result in the `content` field and puts the content_type in `content_type`
  let static_files = StaticFile::new();

  let router = Router::new().route("/*file", get(|uri: Uri| {
      match static_files.get(&uri.path()) {
          Some(file) => (
              StatusCode::OK,
              [(CONTENT_TYPE, file.content_type)],
              file.content,
          ),
          None => (
              StatusCode::NOT_FOUND,
              [(CONTENT_TYPE, "text/html; charset=utf-8")],
              "not found",
          ),
      }
  }));

  let addr = "127.0.0.1:9001".parse().unwrap();
  let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
  println!("Listening on {}", ip);
  axum::serve(listener, router).await.unwrap();
}
```

If you need to reference the static files later there is also a convenience function
the uses `std::sync::OnceLock`:

```rust
#[tokio::main]
async fn main() {
  StaticFiles::once() // loads the static files into a static OnceLock
}

fn render(inner: impl Render + 'static) -> Html {
  // reuse once anywhere
  let static_files = StaticFile::once();
  html::render((
      doctype("html"),
      html((
          head((
              link.href(static_files.tailwind).rel("stylesheet"),
              script.src(static_files.htmx).defer(),
          )),
          body(inner),
      )),
  ))
}
```

