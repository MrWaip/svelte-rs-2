App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button>go</button>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state("x"), "value");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var button = $.sibling(div, 2);
	$.template_effect(() => $.set_attribute(div, "foo", `a${$.get(value) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.set(value, $.get(value) + "!");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
