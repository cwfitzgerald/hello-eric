.PHONY: clean

hello-eric.zip:
	cargo build --release
	cp target/release/hello-eric hello-eric
	tar cJf hello-eric.tar.xz src hello-eric Makefile Cargo.lock Cargo.toml

clean: 
	rm hello-eric hello-eric.tar.xz
