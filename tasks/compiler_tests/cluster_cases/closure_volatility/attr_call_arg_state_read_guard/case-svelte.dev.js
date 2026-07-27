App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button>go</button>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var button = $.sibling(div, 2);
	$.template_effect(($0) => $.set_attribute(div, "title", $0), [() => String($.get(count))]);
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
