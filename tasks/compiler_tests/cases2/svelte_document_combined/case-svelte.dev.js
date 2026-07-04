App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let el = $.tag($.state(null), "el");
	function handleKeydown(e) {
		console.log(...$.log_if_contains_state("log", "keydown", e.key));
	}
	var $$exports = { ...$.legacy_api() };
	$.event("keydown", $.document, handleKeydown);
	$.bind_active_element(function set($$value) {
		$.set(el, $$value, true);
	});
	return $.pop($$exports);
}
