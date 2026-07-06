import * as $ from "svelte/internal/server";
let tmp = {
	a: 1,
	b: 2
}, a = tmp.a, b = tmp.b;
let tmp_1 = [3, 4], $$array = $.to_array(tmp_1, 2), c = $$array[0], d = $$array[1];
export function bump() {
	a++;
	c++;
}
export function read() {
	return a + b + c + d;
}
