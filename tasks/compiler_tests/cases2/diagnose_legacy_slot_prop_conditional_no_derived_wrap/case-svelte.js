import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let flag = $.prop($$props, "flag", 8, false);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", { get title() {
		return flag() ? "A" : "B";
	} }, null);
	$.append($$anchor, fragment);
}
