App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let innerWidth = $.tag($.state(0), "innerWidth");
	let innerHeight = $.tag($.state(0), "innerHeight");
	let outerWidth = $.tag($.state(0), "outerWidth");
	let outerHeight = $.tag($.state(0), "outerHeight");
	let devicePixelRatio = $.tag($.state(1), "devicePixelRatio");
	var $$exports = { ...$.legacy_api() };
	$.bind_window_size("innerWidth", function set($$value) {
		$.set(innerWidth, $$value, true);
	});
	$.bind_window_size("innerHeight", function set($$value) {
		$.set(innerHeight, $$value, true);
	});
	$.bind_window_size("outerWidth", function set($$value) {
		$.set(outerWidth, $$value, true);
	});
	$.bind_window_size("outerHeight", function set($$value) {
		$.set(outerHeight, $$value, true);
	});
	$.bind_property("devicePixelRatio", "resize", $.window, function set($$value) {
		$.set(devicePixelRatio, $$value, true);
	});
	return $.pop($$exports);
}
