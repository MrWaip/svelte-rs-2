import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let flag = $.prop($$props, "flag", 8, false);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", { get title() {
		return flag() ? "A" : "B";
	} }, null);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
