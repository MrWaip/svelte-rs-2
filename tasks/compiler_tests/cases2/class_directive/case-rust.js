import * as $ from "svelte/internal/client";
var root = $.template(`<div>Lorem</div>`);
export default function App($$anchor) {
	let absolute = $.state(undefined);
	let visible = $.state(undefined);
	let unchanged = undefined;
	let untouched = undefined;
	const staticClass = true;
	$.set(visible, 12);
	$.set(absolute, true);
	var div = root();
	$.append($$anchor, div);
}
