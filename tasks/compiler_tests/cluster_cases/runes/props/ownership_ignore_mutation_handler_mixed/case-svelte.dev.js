App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let a = $.prop($$props, "a", 7), b = $.prop($$props, "b", 7);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		//svelte-ignore ownership_invalid_mutation
		a().x = 1;
		$$ownership_validator.mutation("b", ["b", "x"], b().x = 2, 9, 2);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
