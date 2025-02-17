# Installation

You need to do this once and it will be installed in your system.

```bash
cargo install --locked cargo-pgrx
```

And also this :

```bash
cargo pgrx init
```

Then you can create a new project with this command :

```bash
cargo pgrx new THE_NAME_OF_YOUR_PROJECT
```

To check that everything works:

```bash
cargo pgrx run
```

If you get the following error :

```bash
       Using DefaultFeature("pg13") and `pg_config` from /home/visanim/.pgrx/13.19/pgrx-install/bin/pg_config
    Building extension with features pg13
     Running command "/home/visanim/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo" "build" "--lib" "--features" "pg13" "--no-default-features" "--message-format=json-render-diagnostics"
error: rustc 1.78.0 is not supported by the following package:
  home@0.5.11 requires rustc 1.81
Either upgrade rustc or select compatible dependency versions with
`cargo update <name>@<current-ver> --precise <compatible-ver>`
where `<compatible-ver>` is the latest version supporting rustc 1.78.0
```

You need to update Rust using the following command :

```bash
rustup update
```