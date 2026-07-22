import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>a</div>`);
export default function App($$anchor) {
	let arr = $.proxy({ test: () => {} });
	var div = root();
	$.attribute_effect(div, ($0) => ({ ...$0 }), [() => $.snapshot(arr)]);
	$.append($$anchor, div);
}
