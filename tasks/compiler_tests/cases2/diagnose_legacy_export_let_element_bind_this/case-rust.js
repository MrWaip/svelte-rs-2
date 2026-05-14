import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let elRef = $.prop($$props, "elRef", 12, undefined);
	var div = root();
	$.bind_this(div, ($$value) => elRef($$value), () => elRef());
	$.append($$anchor, div);
}
