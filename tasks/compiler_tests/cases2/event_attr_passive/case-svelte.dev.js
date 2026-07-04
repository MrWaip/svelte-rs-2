App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Touch</button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handler() {
		console.log("touch");
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("touchstart", button, handler, void 0, true);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["touchstart"]);
