App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let obj = $.prop($$props, "obj", 15);
	function sync(cb) {
		cb($$ownership_validator.mutation("obj", ["obj", "field"], obj(obj().field = obj().other, true), 4, 5));
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return sync(() => {});
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
