import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	const value = $.derived_safe_equal(() => $$slotProps.item);
	$.slot(node, $$props, "default", {}, null);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
