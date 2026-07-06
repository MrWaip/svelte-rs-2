import * as $ from "svelte/internal/server";
import { a } from "mod-a";
function first() {
	return a;
}
import { b } from "mod-b";
var cache = new Map();
function second() {
	return b;
}
export { first, second };
