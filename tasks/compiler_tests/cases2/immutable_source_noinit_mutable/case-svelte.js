import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor) {
	let el = $.mutable_source();
	var div = root();
	$.bind_this(div, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, div);
}
