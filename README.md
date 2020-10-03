# rocket-yew-todo

 todo sample app written in Rust with Yew and a Rocket API back-end. Yew sample code from Yew Todo example: <https://github.com/yewstack/yew/tree/v0.17/examples/todomvc>.  The Rocket server uses MongoDb as a data store.

## Development

You'll need to install the following:

- cargo <https://doc.rust-lang.org/cargo/getting-started/installation.html>.
- wasm-pack <https://yew.rs/docs/en/getting-started/project-setup/using-wasm-pack>.
- just <https://github.com/casey/just>.

To build the Yew app and run the Rocket server:

`just build-app && just run`

You can also just run the Rocket server using `just run`.

## Code Walkthrough

```plaintext
.
├── dist
│   ├── css
│   │   ├── base.css
│   │   └── index.css
│   ├── index.html
│   ├── package.json
│   ├── wasm_bg.wasm
│   ├── wasm_bg.wasm.d.ts
│   ├── wasm.d.ts
│   └── wasm.js
├── justfile
├── README.md
├── src
│   ├── rocket-app
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── main.rs
│   │       └── repositories
│   │           ├── mod.rs
│   │           └── task.rs
│   └── yew-app
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── css
│       │   ├── base.css
│       │   └── index.css
│       ├── index.html
│       └── src
│           ├── lib.rs
│           └── services
│               ├── mod.rs
│               └── task.rs
```

The `dist` folder will be built by the `yew-app` project when you run `just build-app`.  It will hold all of the static assets for your Yew application.  The Rocket server will serve this at the root as a static directory.  You can also use this folder to deploy its contents to a CDN and host it as a static website.

`rocket-app` contains all of the source files to run the Rocket API server.  You can run this using `just run`.  It connects to a MongoDb instance to retreive and store data.  You need to supply the following environment variables at runtime to connect:

```bash
export MONGODB_URI="<Mongo Uri Connection String>"
export MONGODB_DB="<database name>"
export MONGODB_COLLECTION="<collection name>"
```

`yew-sample` is the Yew project.  It contains the Rust crate that compiles to webassembly, as well as any static assets needed to make the page work such as CSS and HTML.  You can build the Yew app using `just build-app`.
