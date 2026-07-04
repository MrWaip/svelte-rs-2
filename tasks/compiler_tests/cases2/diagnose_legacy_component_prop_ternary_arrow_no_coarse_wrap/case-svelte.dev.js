import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let isButton = $.prop($$props, "isButton", 8);
	let onAction = $.prop($$props, "onAction", 8);
	let item = $.prop($$props, "item", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	{
		let $0 = $.derived_safe_equal(() => isButton() ? () => onAction()(item()) : undefined);
		$.add_svelte_meta(() => Child($$anchor, { get onclick() {
			return $.get($0);
		} }), "component", App, 7, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
