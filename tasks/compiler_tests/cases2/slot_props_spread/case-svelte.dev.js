import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let props = { foo: "bar" };
	let item = "hello";
	let extra = "world";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "footer", $.spread_props({
		item,
		extra
	}, () => props), null);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
