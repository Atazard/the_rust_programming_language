# the_rust_programming_language
My journey through the book The Rust programming language.

Most chapters of the book have different excersices to ease you into Rust.
This repo contains every project created to test stuff for each chapter of the book.

Notes:

- Create a new project with cargo:  
cargo new project_name

- Update settings.json so rust-analyzer can work properly:  
add ".\\project_name\\Cargo.toml"

- Live reloading with cargo-watch  
cargo watch -q -c -w src/ -x 'run -q'
  - q: quiet, supresses the output of cargo watch
  - c: clears the screen
  - w: specify files or folders to watch (in this case only the source directory)
  - x: specify cargo command to execute (run quiet)

Some usefull crates:
  - serde
  - serde_json
  - chrono
  - itertools
  - tokio
  - pretty_env_logger
  - derive_builder