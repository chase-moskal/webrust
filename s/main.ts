
import {greet} from "../rust/pkg/rust.js"

greet("Chase", (...args: any[]) => console.log(...args))
