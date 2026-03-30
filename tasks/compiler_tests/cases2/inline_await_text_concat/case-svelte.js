import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var response;
	var $$promises = $.run([async () => response = await fetch("/api")]);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, `Hello ${$0 ?? ""}!`), void 0, [() => response.text()], [$$promises[0]]);
	$.append($$anchor, p);
}
