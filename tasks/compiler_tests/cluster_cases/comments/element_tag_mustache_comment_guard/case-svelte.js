import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_class(div, 1, $.clsx(foo));
	$.append($$anchor, div);
}
