import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var a, b, c;
	var $$promises = $.run([
		() => Promise.resolve(),
		() => a = "a",
		() => Promise.resolve(),
		() => b = "b",
		() => Promise.resolve(),
		() => c = "c"
	]);
	var div = root();
	let styles;
	$.template_effect(() => {
		styles = $.set_style(div, "w: a", styles, { color: c });
		$.set_attribute(div, "title", b);
	}, void 0, void 0, [
		$$promises[1],
		$$promises[5],
		$$promises[3]
	]);
	$.append($$anchor, div);
}
