App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let isOnline = $.tag($.state(true), "isOnline");
	var $$exports = { ...$.legacy_api() };
	$.bind_online(function set($$value) {
		$.set(isOnline, $$value, true);
	});
	return $.pop($$exports);
}
