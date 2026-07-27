import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>x</p>`);
export default function App($$anchor) {
	function fn() {
		return 1;
	}
	var p = root();
	$.attribute_effect(p, ($0, $1) => ({
		...{},
		class: $1,
		id: $0
	}), [() => fn()], [() => "neato"]);
	$.append($$anchor, p);
}
