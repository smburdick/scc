all: clean
	mkdir out
	touch out/a.s
	cargo build

run-comp: all
	cargo run c/return_comp_not_neg_1.c
	cd out
	gcc out/a.s -o out/out
	-out/out

clean:
	rm -rf out