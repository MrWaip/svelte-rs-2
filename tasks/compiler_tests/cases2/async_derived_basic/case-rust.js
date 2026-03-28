import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let url = "/api";
	var data;
	var $$promises = $.run([async () => data = $.derived(() => await fetch(url))]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(data)), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
}
