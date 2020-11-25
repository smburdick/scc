# scc (Sam's C Compiler)
A C compiler (will compile to x86 or RISC) written in Rust.
## Run it (to reveal the meaning of life)
```
mkdir out
touch out/a.s
cargo run
cd out
gcc a.s
./a.out
echo $?
```
## TODOs
* Complete remainder of the blog exercises
* Move compilation/running instructions to Makefile