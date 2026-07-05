App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({
		a: 1,
		b: 2
	}), "obj");
	let snap = $.snapshot(obj);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
