import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = 0;
	function update() {
		let x, y;
		[x, y] = [1, 2];
		return x + y;
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	button.textContent = "0";
	$.event("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
