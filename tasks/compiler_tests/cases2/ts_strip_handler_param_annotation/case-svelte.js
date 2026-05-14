import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>x</div>`);
export default function App($$anchor) {
	var div = root();
	$.delegated("click", div, (event) => event.stopPropagation());
	$.append($$anchor, div);
}
$.delegate(["click"]);
