App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let test = $.prop($$props, "test", 7);
	function go() {
		//svelte-ignore ownership_invalid_mutation
		test().test = Math.random();
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
