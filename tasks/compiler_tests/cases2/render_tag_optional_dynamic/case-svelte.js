import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let show = $.prop($$props, "show", 3, null);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, () => show() ?? $.noop, () => "hello");
	$.append($$anchor, fragment);
}
