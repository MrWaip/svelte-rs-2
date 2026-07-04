App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handler() {
		console.log("touch move");
	}
	var $$exports = { ...$.legacy_api() };
	$.event("touchmove", $.window, handler, void 0, true);
	return $.pop($$exports);
}
