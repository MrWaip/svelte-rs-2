import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var response;
	var $$promises = $.run([async () => response = await fetch("/api")]);
	var div = root();
	$.template_effect(($0) => $.set_attribute(div, "title", $0), void 0, [() => response.text()], [$$promises[0]]);
	$.append($$anchor, div);
}
