import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option value="a">A</option><option value="b">B</option></select>`);
export default function App($$anchor) {
	let selected = $.state("a");
	var select = root();
	$.next();
	$.reset(select);
	$.bind_select_value(select, () => $.get(selected), ($$value) => $.set(selected, $$value));
	$.append($$anchor, select);
}
