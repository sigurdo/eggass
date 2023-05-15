# seba2 - Scientific Egg Boiling Assistant 2

[Go to app](https://seba2.duckdns.org/)

seba2 is a web app that helps you boil your eggs with scientific precision, based on [the research by Charles D. H. Williams](https://newton.ex.ac.uk/teaching/CDHW/egg/) at the University of Exeter.

The app is written in [Rust](https://www.rust-lang.org/) and compiled to webassembly with [wasm-pack](https://github.com/rustwasm/wasm-pack). The repository structure is based on [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template).

## Build

Compile Rust code with

```
wasm-pack build
```

Build web app with

```
cd www
npm run build
```

Debug web app with

```
cd www
npm run start
```

and open [http://localhost:8080](http://localhost:8080) in your browser.

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under [WTFPL](LICENSE)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

