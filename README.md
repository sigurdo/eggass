# Eggass - Scientific egg boiling assistant

[Go to app](https://eggass.sigurdt.no/)

Eggass is a web app that helps you boil your eggs with scientific precision, based on [the research by Charles D. H. Williams](https://newton.ex.ac.uk/teaching/CDHW/egg/) at the University of Exeter.

The app is written in [Rust](https://www.rust-lang.org/) and compiled to webassembly with [wasm-pack](https://github.com/rustwasm/wasm-pack). The repository structure is based on [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template).

## Build

Compile Rust code with

```
wasm-pack build
```

Build web app with

```
cd www
npm install
npm run build
cd ..
```

Debug web app with

```
cd www
npm run start
```

and open [http://localhost:8080](http://localhost:8080) in your browser.

## License

Licensed under [WTFPL](LICENSE)
