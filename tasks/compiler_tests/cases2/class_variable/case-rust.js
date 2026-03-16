import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let cls = "primary";
	var div = root();
	$.set_class(div, 1, $.clsx(cls));
	$.append($$anchor, div);
}
