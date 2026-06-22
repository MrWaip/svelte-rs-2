import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let value;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	const value = $.derived_safe_equal(() => $$slotProps.item);
	$.slot(node, $$props, "default", {}, null);
	$.append($$anchor, fragment);
}
