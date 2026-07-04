App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let el = $.tag($.state(null), "el");
	let state = $.tag($.state("visible"), "state");
	var $$exports = { ...$.legacy_api() };
	$.bind_active_element(function set($$value) {
		$.set(el, $$value, true);
	});
	$.bind_property("fullscreenElement", "fullscreenchange", $.document, function set($$value) {
		$.set(el, $$value, true);
	});
	$.bind_property("pointerLockElement", "pointerlockchange", $.document, function set($$value) {
		$.set(el, $$value, true);
	});
	$.bind_property("visibilityState", "visibilitychange", $.document, function set($$value) {
		$.set(state, $$value, true);
	});
	return $.pop($$exports);
}
