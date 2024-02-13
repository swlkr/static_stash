# Static stash

Static stash provides an easy way to declare and embed your static files!

```sh
cargo add static_stash # sorry not on crates.io yet
```

## Declare your static files

This will embed the static files in your binary at compile time with `include_str!`.
It will try to find the files starting from the root of your project: `CARGO_MANIFEST_DIR`.

```rust
use static_stash::{StaticFiles, Js, Css};

#[derive(StaticFiles)]
struct StaticFile {
  #[file("/htmx.js")]
  htmx: Js,
  #[file("/tailwind.css")]
  tailwind: Css
}
```

## Serve them

```rust
use axum::routing::get;
use axum::{Server, Router};
use axum::response::IntoResponse;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/*file", static_files);
    serve("127.0.0.1:9001", router).await.unwrap();
}

async fn static_files(uri: Uri) -> impl IntoResponse {
    match StaticFile::get(uri.path()) {
        Some((content_type, bytes)) => (
            StatusCode::OK,
            [(CONTENT_TYPE, content_type)],
            bytes,
        ),
        None => (
            StatusCode::NOT_FOUND,
            [(CONTENT_TYPE, "text/html; charset=utf-8")],
            "not found".as_bytes(),
        ),
    }
}

async fn serve(ip: &str, router: Router) {
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    println!("Listening on {}", ip);
    axum::serve(listener, router).await.unwrap();
}
```

## Reference them

```rust
fn render(inner: Element) -> String {
    let file = StaticFile::new();

    render((
        doctype("html"),
        html((
            head((
              link.href(file.tailwind).rel("stylesheet"),
              script.src(file.htmx).defer(),
            )),
            body(inner),
        )),
    ))
}
```

