import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = 0;
	function go() {
		let o = { a: 0 };
		[o.a] = [1];
		return o.a;
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	button.textContent = "0";
	$.event("click", button, go);
	$.append($$anchor, button);
	return $.pop($$exports);
}
