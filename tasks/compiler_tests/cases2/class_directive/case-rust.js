import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>Lorem</div>`);
export default function App($$anchor) {
	let absolute = $.state(void 0);
	let visible = $.state(void 0);
	let unchanged = void 0;
	let untouched = void 0;
	const staticClass = true;
	$.set(visible, 12);
	$.set(absolute, true);
	var div = root();
	$.append($$anchor, div);
}
