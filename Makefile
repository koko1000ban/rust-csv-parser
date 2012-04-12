
all:
	mkdir -p ./lib
	rustc --lib ./csv.rc --out-dir ./lib

test: all
	mkdir -p ./build
	rustc --test ./csv.rc --out-dir ./build
	rustc --test -L ./lib ./test/simple.rs --out-dir ./build
	find ./build -perm -u+x -type f -exec {} \;

clean:
	rm -rf ./lib
	rm -rf ./build
