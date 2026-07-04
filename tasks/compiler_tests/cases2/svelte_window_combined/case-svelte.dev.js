App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let scrollY = $.tag($.state(0), "scrollY");
	function handleResize() {
		console.log("resized");
	}
	var $$exports = { ...$.legacy_api() };
	$.event("resize", $.window, handleResize);
	$.bind_window_scroll("y", function get() {
		return $.get(scrollY);
	}, function set($$value) {
		$.set(scrollY, $$value, true);
	});
	return $.pop($$exports);
}
