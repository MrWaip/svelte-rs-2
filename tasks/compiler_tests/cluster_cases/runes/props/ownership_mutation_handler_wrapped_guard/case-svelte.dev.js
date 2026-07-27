App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let test = $.prop($$props, "test", 7);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return $$ownership_validator.mutation("test", ["test", "test"], test().test = Math.random(), 5, 23);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
