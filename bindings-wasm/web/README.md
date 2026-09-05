At this repository root

```
$ wasm-pack build bindings-wasm --target web --out-dir web/pkg
$ python -m http.server 8000 --directory bindings-wasm/web
```
