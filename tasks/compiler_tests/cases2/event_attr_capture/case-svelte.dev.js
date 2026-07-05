App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Click</button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handler() {
		console.log("click capture");
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, handler, true);
	$.append($$anchor, button);
	return $.pop($$exports);
}
