import * as $ from "svelte/internal/client";
var root = $.template(`<div><div><div><div></div></div></div></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
