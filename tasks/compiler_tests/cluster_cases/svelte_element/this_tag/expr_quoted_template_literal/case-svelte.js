import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let size = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => `h${size}`, false);
	$.append($$anchor, fragment);
}
