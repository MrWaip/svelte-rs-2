import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.html(node, () => content, void 0, void 0, true);
	$.append($$anchor, fragment);
}
