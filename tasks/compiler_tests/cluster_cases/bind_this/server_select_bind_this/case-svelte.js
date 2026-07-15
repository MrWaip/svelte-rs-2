import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option></select>`);
export default function App($$anchor) {
	let ref;
	let val = $.state("a");
	var select = root();
	$.bind_this(select, ($$value) => ref = $$value, () => ref);
	$.bind_select_value(select, () => $.get(val), ($$value) => $.set(val, $$value));
	$.append($$anchor, select);
}
