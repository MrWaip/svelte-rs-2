import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let item = "hello";
	function get_item() {
		return item;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived_safe_equal(() => $.untrack(get_item));
		$.slot(node, $$props, "default", { get item() {
			return $.get($0);
		} }, null);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
