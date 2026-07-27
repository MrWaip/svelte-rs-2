App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>toggle</button> <!doctype/>`, 1), App[$.FILENAME], [[5, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let kind = $.tag($.state("html"), "kind");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var _doctype = $.sibling(button, 2);
	$.template_effect(() => $.set_attribute(_doctype, "html", $.get(kind)));
	$.delegated("click", button, function click() {
		return $.set(kind, "xml");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
