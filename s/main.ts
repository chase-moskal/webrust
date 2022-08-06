
import {greet, sum} from "../rust/pkg/rust.js"

greet("Chase", (...args: any[]) => console.log(...args))

const a = 2
const b = 3

console.log("sum of a and b:", sum(a, b))
