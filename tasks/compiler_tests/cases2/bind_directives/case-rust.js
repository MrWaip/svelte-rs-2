import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input>  <input>`, 1);
export default function App($$anchor) {
	let value = "";
	let name = "";
	var fragment = root();
	$.append($$anchor, fragment);
}
