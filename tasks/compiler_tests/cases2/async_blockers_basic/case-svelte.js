import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let x = 1;
	var data, y;
	var $$promises = $.run([async () => data = await fetch("/api"), () => y = data.value]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, y), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, p);
}
