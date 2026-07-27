import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var a, b;
	var $$promises = $.run([() => Promise.resolve(), () => {
		a = "a";
		b = "b";
	}]);
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, {
		one: a,
		two: b
	}), void 0, void 0, [$$promises[1], $$promises[1]]);
	$.append($$anchor, div);
}
