import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let item = $.prop($$props, "item", 8, "hello");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", { get item() {
		return item();
	} }, null);
	$.append($$anchor, fragment);
}
