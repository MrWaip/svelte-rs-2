App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!-- svelte-ignore a11y_no_static_element_interactions --> <div>click</div>`, 1), App[$.FILENAME], [[2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	var div = $.sibling(node, 2);
	$.delegated("click", div, function click() {});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
