import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var value;
	var $$promises = $.run([async () => value = await fetch("/api")]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), void 0, [() => value], [$$promises[0]]);
	$.append($$anchor, p);
}
