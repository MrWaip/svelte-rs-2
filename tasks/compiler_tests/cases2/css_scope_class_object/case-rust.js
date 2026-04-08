import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="svelte-az1y0o">content</div>`);
export default function App($$anchor) {
	let active = false;
	let big = false;
	var div = root();
	$.set_class(div, 1, $.clsx({
		active,
		big
	}), null, {}, { extra: active });
	$.append($$anchor, div);
}
