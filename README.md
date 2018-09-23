# wasm-fuzzy-search
A simple fuzzy search function written in Rust and compiled to WASM

### Purpose
Playing around with the [rustwasm](https://github.com/rustwasm/team) toolchains.

#### Development
First please follow these [steps](https://rustwasm.github.io/book/game-of-life/setup.html) to install Rust & deps.

Second:
```
$ git clone https://github.com/andrei-cacio/wasm-fuzzy-search
$ cd wasm-fuzzy-search
$ bash build.sh
$ cd pkg
$ npm link
```

Now that we have locally built and linked our NPM module, we will need an app to use it:

```
$ git clone https://github.com/andrei-cacio/fuzzy-search-app
$ cd fuzzy-search-app
$ npm i
$ npm start
```

Now we can go to [http://localhost:8080](http://localhost:8080) and have some fun.

#### Production
TODO: publish to npm
