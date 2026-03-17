import * as $ from "svelte/internal/client";
var root = $.from_html(`<select></select>`);
export default function App($$anchor) {
	let selected = $.state("a");
	var select = root();
	$.bind_select_value(select, () => $.get(selected), ($$value) => $.set(selected, $$value));
	$.append($$anchor, select);
}
