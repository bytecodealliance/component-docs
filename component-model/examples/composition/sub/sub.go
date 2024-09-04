package main

//go:wasm-module sub
//export docs:subtractor/sub#sub
func sub(x, y int32) int32 {
	return x - y
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
