Misery Loves Company Bot
========================

Vetting dependancies is hard. Let the hivemind do it for you.

This tool checks your cargo.toml to determine if any of your dependancies are unpopular.
Any libs with less than 100000 downloads will be marked unsafe, and prevent the build.

Usage
-----

To use: 

Cargo.toml:
```toml
[build-dependencies]
misery_loves_company_bot = { git = "https://github.com/cameroncros/misery_loves_company_bot.git" }
```

build.rs:
```rust
use misery_loves_company_bot::check_deps;

fn main() {
    check_deps()
}
```

Testing
-------

I haven't tested it, so its probably good.

Contributing
------------

Nah, do something better with your time.
