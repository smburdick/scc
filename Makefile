all: clean
	mkdir out
	touch out/a.s
	cargo build

run-compl: all
	cargo run
	cd out
	ls -al
	gcc out/a.s -o out/out
	-out/out

clean:
	rm -rf out