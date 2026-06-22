import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let container = $.prop($$props, "container", 28, () => ({}));
	let paths = $.prop($$props, "paths", 24, () => ["a"]);
	$.init();
	var div = root();
	$.bind_this(div, ($$value) => container(container()[paths()[0]] = $$value, true), () => container()?.[paths()[0]]);
	$.append($$anchor, div);
	$.pop();
}
