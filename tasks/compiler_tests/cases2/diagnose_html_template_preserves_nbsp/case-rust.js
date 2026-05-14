import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>пополняем&nbsp;всегда</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
