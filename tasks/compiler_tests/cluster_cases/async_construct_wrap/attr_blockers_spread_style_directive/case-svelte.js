import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	var color;
	var $$promises = $.run([() => Promise.resolve(), () => color = "red"]);
	var div = root();
	$.attribute_effect(div, () => ({
		...$$props.rest,
		[$.STYLE]: { color }
	}), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, div);
}
