import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let entry = "hello";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", {}, null);
	$.append($$anchor, fragment);
}
