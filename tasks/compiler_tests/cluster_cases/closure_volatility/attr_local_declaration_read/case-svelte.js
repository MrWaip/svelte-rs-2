import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	var div = root();
	$.set_attribute(div, "title", [() => {
		let q = 1;
		return q;
	}]);
	$.append($$anchor, div);
}
