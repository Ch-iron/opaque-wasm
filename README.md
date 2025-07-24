## opaque-wasm

An implementation of the OPAQUE key exchange protocol in WASM(WebAssembly). This implementation is based on the [opaque-ke](https://github.com/facebook/opaque-ke).

### Build for simple example test
```
wasm-pack build --target nodejs
```

### JS simple Example Test
```
cd tests/js-test
npm install
node test.mjs
```

### Build for client
```
cd scripts
./build_client.sh
```

### Build for server
```
cd scripts
./build_server.sh
```

### Build for client & server simultaneously
```
cd scripts
./build_all.sh
```

### E2E Test
```
cd tests/e2e
npm install
npm run build
npm start
```