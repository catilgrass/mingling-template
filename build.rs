fn main() {
    build_scripts();
}

/// Generate completion scripts
fn build_scripts() {
    // use `env!("CARGO_PKG_NAME")`
    // note: if your binary name differs from the crate name, specify it explicitly
    mingling::build::build_comp_scripts(
        // Your binary name:
        env!("CARGO_PKG_NAME"),
    )
    .unwrap();
}
