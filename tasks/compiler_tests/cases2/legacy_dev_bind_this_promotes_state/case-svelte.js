import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor) {
	let target = $.mutable_source();
	var div = root();
	$.bind_this(div, ($$value) => $.set(target, $$value), () => $.get(target));
	$.append($$anchor, div);
}
