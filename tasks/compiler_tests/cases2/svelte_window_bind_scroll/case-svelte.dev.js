App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let scrollX = $.tag($.state(0), "scrollX");
	let scrollY = $.tag($.state(0), "scrollY");
	var $$exports = { ...$.legacy_api() };
	$.bind_window_scroll("x", function get() {
		return $.get(scrollX);
	}, function set($$value) {
		$.set(scrollX, $$value, true);
	});
	$.bind_window_scroll("y", function get() {
		return $.get(scrollY);
	}, function set($$value) {
		$.set(scrollY, $$value, true);
	});
	return $.pop($$exports);
}
