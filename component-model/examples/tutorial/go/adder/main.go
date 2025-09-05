//go:generate go tool wit-bindgen-go generate --world adder --out internal ./docs:adder@0.1.0.wasm

package main

import (
	"example.com/internal/docs/adder/add"
)

func init() {
	add.Exports.Add = func(x uint32, y uint32) uint32 {
		return x + y
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
