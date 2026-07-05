import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, () => $$props.row);
	$.append($$anchor, fragment);
}
