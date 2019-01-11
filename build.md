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


### Deployment
Depending on where the project will be hosted you should update the configuration in [vue.config.js](vue.config.js)

```JS
publicPath: process.env.NODE_ENV === 'production'
    ? '' //here the relative path e.g. if you plan to host it in github pages [username].github.io/[repoName], the value if true should be [repoName]
    : ''
```