import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var value;
	var $$promises = $.run([() => Promise.resolve(), () => value = "value"]);
	var div = root();
	$.template_effect(() => $.set_class(div, 1, $.clsx(value)), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, div);
}
