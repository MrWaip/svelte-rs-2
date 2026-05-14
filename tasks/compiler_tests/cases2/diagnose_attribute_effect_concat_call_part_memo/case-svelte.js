import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let size = 1;
	let arr = $.proxy([]);
	function joinClasses(a) {
		return a.join(" ");
	}
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...{ id: "x" },
		class: `size_1 ${$0 ?? ""}`
	}), [() => joinClasses(arr)]);
	$.append($$anchor, div);
}
