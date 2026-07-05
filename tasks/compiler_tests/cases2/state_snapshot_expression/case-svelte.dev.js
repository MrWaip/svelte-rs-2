App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ a: 1 }), "obj");
	console.log(...$.log_if_contains_state("log", $.snapshot(obj)));
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
