import * as $ from "svelte/internal/client";
let tmp = {
	a: 1,
	b: 2
}, a = $.state($.proxy(tmp.a)), b = $.proxy(tmp.b);
let tmp_1 = [3, 4], $$array = $.derived(() => $.to_array(tmp_1, 2)), c = $.state($.proxy($.get($$array)[0])), d = $.proxy($.get($$array)[1]);
export function bump() {
	$.update(a);
	$.update(c);
}
export function read() {
	return $.get(a) + b + $.get(c) + d;
}
