App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>content</div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handler() {
		console.log("got pointer capture");
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.event("gotpointercapture", div, handler);
	$.append($$anchor, div);
	return $.pop($$exports);
}
