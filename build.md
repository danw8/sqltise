# Install these
2. ```rustup target add wasm32-unknown-unknown```
3. ```cargo install wasm-bindgen-cli```

# Build steps
## Rust

## front end
```
cd sqlitise_web
```

```
wasm-bindgen --target web ..\target\wasm32-unknown-unknown\debug\sqltise_web.wasm --out-dir pkg
```