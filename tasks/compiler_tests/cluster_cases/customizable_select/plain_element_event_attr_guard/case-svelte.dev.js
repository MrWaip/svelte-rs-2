App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button class="y">b</button>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rest = $.tag_proxy($.proxy({}), "rest");
	function onclick() {}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	$.attribute_effect(div, () => ({
		...rest,
		onclick,
		class: "x"
	}));
	var button = $.sibling(div, 2);
	$.delegated("click", button, onclick);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
