App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ a: 1 }), "obj");
	function report() {
		console.log(...$.log_if_contains_state("log", obj));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, report);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
