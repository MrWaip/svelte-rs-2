import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const onClose = () => {};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "header", { onClose }, null);
	$.append($$anchor, fragment);
}
