import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>neato</p>`);
export default function App($$anchor) {
	var p = root();
	$.attribute_effect(p, ($0) => ({
		...{},
		class: $0
	}), void 0, [() => "neato"]);
	$.append($$anchor, p);
}
