run: 
	cd src/rocket-app && rustup run nightly cargo run && cd -

build-app:
	rm -rf dist && \
	mkdir dist && \
	cd src/yew-app && \
	cp -r css ../../dist/css && \
	cp index.html ../../dist/index.html && \
	wasm-pack build --target web --out-name wasm --out-dir ../../dist && \
	cd -