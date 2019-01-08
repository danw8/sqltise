# csv2sql

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

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

# steps I took
1. npm install -g @vue/cli
2. rustup target add wasm32-unknown-unknown --toolchain nightly
3. cargo +nightly install wasm-bindgen-cli


4. cargo +nightly build --target wasm32-unknown-unknown
5. wasm-bindgen target/wasm32-unknown-unknown/debug/csv2sql.wasm --out-dir .\src\rust\
4. npm run serve to build and server the vue.js app