import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var msg;
	var $$promises = $.run([() => Promise.resolve(), () => msg = "hi"]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, "hi"), void 0, void 0, [$$promises[1]]);
	$.append($$anchor, p);
}
