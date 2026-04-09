import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>content</div>`);
export default function App($$anchor) {
	let active = false;
	let big = false;
	var div = root();
	$.set_class(div, 1, $.clsx({
		active,
		big
	}), "svelte-az1y0o", {}, { extra: active });
	$.append($$anchor, div);
}
