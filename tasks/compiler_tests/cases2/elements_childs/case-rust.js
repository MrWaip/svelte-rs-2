import * as $ from "svelte/internal/client";
var root = $.template(`<div>text only</div> <div></div> <div></div> <div><div>more nested</div> <div>more nested</div> <div>more nested</div></div> <div><!></div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.append($$anchor, fragment);
}
