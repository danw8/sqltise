# Install these
1. ```npm install -g @vue/cli```
2. ```rustup target add wasm32-unknown-unknown --toolchain nightly```
3. ```cargo +nightly install wasm-bindgen-cli```

# Build steps
## Rust
1. ```cd csv2sql```
2. ```cargo +nightly build --target wasm32-unknown-unknown```
3. ```wasm-bindgen target/wasm32-unknown-unknown/debug/csv2sql.wasm --out-dir ..\ui\src\csv2sql\```

## vuejs
4. ```cd ../ui```
5. ```npm i```
6. ```npm run serve``` for development
7. ```npm run build``` for production



### Run your tests
```
npm run test
```

### Lints and fixes files
```
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
