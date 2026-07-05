App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button> <div></div>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	let w = $.tag($.state(0), "w");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	let styles;
	$.template_effect(($0) => styles = $.set_style(div, "", styles, $0), [() => ({ left: `${Math.min($.get(x) + 3, $.get(w) - 10)}px` })]);
	$.delegated("click", button, function click() {
		$.update(x);
		$.update(w);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
