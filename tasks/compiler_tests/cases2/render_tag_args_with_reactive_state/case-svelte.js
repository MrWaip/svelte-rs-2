import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let count = 0;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, () => $$props.row, () => count + 1);
	$.append($$anchor, fragment);
}
