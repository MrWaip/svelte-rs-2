App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Click me</button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function handleClick() {
		console.log("clicked");
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, $.preventDefault(handleClick));
	$.append($$anchor, button);
	return $.pop($$exports);
}
