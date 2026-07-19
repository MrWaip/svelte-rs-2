App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function log() {
		console.log($.effect_tracking());
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, log);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
