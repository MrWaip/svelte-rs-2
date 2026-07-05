App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button>toggle</button>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state("red"), "value");
	const getClass = () => $.strict_equals($.get(value), "blue");
	const getValue = () => $.get(value);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	let classes;
	let styles;
	var button = $.sibling(div, 2);
	$.template_effect(($0, $1) => {
		classes = $.set_class(div, 1, "", null, classes, $0);
		styles = $.set_style(div, "", styles, $1);
	}, [() => ({ blue: getClass() }), () => ({ color: getValue() })]);
	$.delegated("click", button, function click() {
		return $.set(value, "blue");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
