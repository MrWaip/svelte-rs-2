import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let item = "hello";
	function get_item() {
		return item;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived_safe_equal(() => $.untrack(get_item));
		$.slot(node, $$props, "default", { get item() {
			return $.get($0);
		} }, null);
	}
	$.append($$anchor, fragment);
}
