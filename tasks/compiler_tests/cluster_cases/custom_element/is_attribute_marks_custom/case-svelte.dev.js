import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button is="my-button"></button>`, 2), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let obj = $.prop($$props, "obj", 8);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.template_effect(() => $.set_custom_element_data(button, "foo", obj()));
	$.append($$anchor, button);
	return $.pop($$exports);
}
