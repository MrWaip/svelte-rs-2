import * as $ from "svelte/internal/client";
let tmp = {
	a: 1,
	b: 2
}, a = $.tag($.state($.proxy(tmp.a)), "a"), b = $.tag_proxy($.proxy(tmp.b), "b");
let tmp_1 = [3, 4], $$array = $.tag($.derived(() => $.to_array(tmp_1, 2)), "[$state iterable]"), c = $.tag($.state($.proxy($.get($$array)[0])), "c"), d = $.tag_proxy($.proxy($.get($$array)[1]), "d");
export function bump() {
	$.update(a);
	$.update(c);
}
export function read() {
	return $.get(a) + b + $.get(c) + d;
}
