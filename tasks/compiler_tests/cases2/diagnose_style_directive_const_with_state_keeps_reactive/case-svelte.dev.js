App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button>change</button>`, 1), App[$.FILENAME], [[8, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const transform = "translateY(1px)";
	let color = $.tag($.state("red"), "color");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	let styles;
	var button = $.sibling(div, 2);
	$.template_effect(() => styles = $.set_style(div, "", styles, {
		transform,
		color: $.get(color)
	}));
	$.delegated("click", button, function click() {
		return $.set(color, "blue");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
