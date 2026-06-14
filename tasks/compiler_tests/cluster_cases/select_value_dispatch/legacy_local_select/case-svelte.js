import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor) {
	let selected = $.mutable_source("a");
	var select = root();
	$.bind_select_value(select, () => $.get(selected), ($$value) => $.set(selected, $$value));
	$.append($$anchor, select);
}
