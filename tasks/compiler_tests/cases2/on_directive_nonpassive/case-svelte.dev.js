App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>Touch</div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handleMove() {
		console.log("move");
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.event("touchmove", div, handleMove, void 0, false);
	$.append($$anchor, div);
	return $.pop($$exports);
}
