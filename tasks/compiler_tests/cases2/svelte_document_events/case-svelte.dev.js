App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handleKeydown(e) {
		console.log(...$.log_if_contains_state("log", "keydown", e.key));
	}
	function handleKeyup(e) {
		console.log("keyup");
	}
	var $$exports = { ...$.legacy_api() };
	$.event("keydown", $.document, handleKeydown);
	$.event("keyup", $.document, handleKeyup);
	$.event("keydown", $.document, $.once(handleKeydown), true);
	return $.pop($$exports);
}
