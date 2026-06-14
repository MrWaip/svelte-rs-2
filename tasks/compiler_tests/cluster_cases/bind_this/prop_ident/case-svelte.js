import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 12);
	var div = root();
	$.bind_this(div, ($$value) => foo($$value), () => foo());
	$.append($$anchor, div);
}
