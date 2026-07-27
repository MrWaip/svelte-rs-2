import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function delay(value) {
		return Promise.resolve(value);
	}
	var attrs;
	var $$promises = $.run([async () => attrs = await delay({ title: "hi" })]);
	var div = root();
	$.attribute_effect(div, () => ({ ...attrs }), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, div);
}
