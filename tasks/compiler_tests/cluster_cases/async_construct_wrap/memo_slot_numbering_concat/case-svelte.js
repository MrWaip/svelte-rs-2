import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>y</div>`);
export default function App($$anchor) {
	function fn() {
		return 1;
	}
	var div = root();
	$.attribute_effect(div, ($0, $1) => ({
		...{},
		title: `a${$1 ?? ""}b`,
		id: $0
	}), [() => fn()], [() => "x"]);
	$.append($$anchor, div);
}
