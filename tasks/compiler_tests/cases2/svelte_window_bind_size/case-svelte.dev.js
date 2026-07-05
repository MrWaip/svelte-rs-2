App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let w = $.tag($.state(0), "w");
	let h = $.tag($.state(0), "h");
	var $$exports = { ...$.legacy_api() };
	$.bind_window_size("innerWidth", function set($$value) {
		$.set(w, $$value, true);
	});
	$.bind_window_size("innerHeight", function set($$value) {
		$.set(h, $$value, true);
	});
	return $.pop($$exports);
}
