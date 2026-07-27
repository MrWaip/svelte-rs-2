import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>y</div>`);
export default function App($$anchor) {
	var div = root();
	let classes;
	$.template_effect(($0) => classes = $.set_class(div, 1, $0, null, classes, { b: true }), void 0, [async () => $.clsx(await "a")]);
	$.append($$anchor, div);
}
