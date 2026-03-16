import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let active = false;
	let highlighted = true;
	const base = "btn";
	var div = root();
	$.set_class(div, 1, $.clsx([base, active && "active"]), null, {}, { highlighted });
	$.append($$anchor, div);
}
