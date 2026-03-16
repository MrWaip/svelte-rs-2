import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let count = $.state(0);
	let items = $.state([
		1,
		2,
		3
	]);
	let empty = void 0;
	let readonly_obj = { x: 1 };
	$.set(count, 10);
	$.set(count, $.get(count) + 5);
	$.set(items, [
		4,
		5,
		6
	]);
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.append($$anchor, div);
}
