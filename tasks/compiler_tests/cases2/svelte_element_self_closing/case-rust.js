import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let tag = "hr";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => tag, false);
	$.append($$anchor, fragment);
}
