import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let content = "<p>safe</p>";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.html(node, () => content);
	$.append($$anchor, fragment);
}
