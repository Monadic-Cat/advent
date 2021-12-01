IDR_SOURCES = $(wildcard src/*.idr)

.PHONY:
run: build/exec/Main
	./build/exec/Main

.PHONY:
clean:
	rm -rf ./build

build/exec/Main: ${IDR_SOURCES}
	idris2 --source-dir src/ src/Main.idr -o Main
