import * as $ from "svelte/internal/client";
import { real } from "mod3";
export function f1(x) {
	console.log(x, real);
}
export function f2(...xs) {
	return xs.length;
}
export const v1 = obj.a?.b.c;
export function f3() {
	counter.n++;
	counter.n = 5;
}
export class A {
	[Symbol.dispose]() {}
	concrete;
	constructor(v) {
		this.concrete = v;
	}
}
