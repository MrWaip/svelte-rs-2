App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function report() {
		console.error("Error: " + $$props.message);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, report);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
