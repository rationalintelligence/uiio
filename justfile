try:
    cargo run --example counter

version := `toml get Cargo.toml package.version --raw`
tag := "v" + version

bump:
    cargo set-version --bump patch

tag:
    git tag {{tag}}
    git push origin {{tag}}

release:
    cargo publish
