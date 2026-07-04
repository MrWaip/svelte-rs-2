App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	let snap = $.snapshot(count);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
