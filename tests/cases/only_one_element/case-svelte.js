import * as $ from "svelte/internal/client";
var root = $.template(`<div>text</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
