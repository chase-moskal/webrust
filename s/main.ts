
import {greet, sum} from "../rust/pkg/rust.js"

{
	console.log("RUST:")
	greet("Chase", (...args: any[]) => console.log(...args))
	const a = 2
	const b = 3
	console.log("sum of a and b:", sum(a, b))
}

{
	console.log("WASM:")
	const wasm = await WebAssembly.instantiateStreaming(fetch("/x/example.wasm"), {})
	const {add} = <any>wasm.instance.exports
	console.log("wasm add:", add(2, 3))
}
