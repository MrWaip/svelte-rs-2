import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "#text";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false);
	$.append($$anchor, fragment);
}
