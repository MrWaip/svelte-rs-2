import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function compute() {
		return "x";
	}
	var a;
	var $$promises = $.run([() => Promise.resolve(), () => a = compute()]);
	var div = root();
	let styles;
	$.template_effect(() => styles = $.set_style(div, `a: ${a ?? ""}`, styles, { width: a }), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, div);
}
