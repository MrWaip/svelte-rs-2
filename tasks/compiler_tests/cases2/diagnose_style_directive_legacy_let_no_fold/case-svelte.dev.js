App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div> <button>bump</button>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	let styles;
	var button = $.sibling(div, 2);
	$.template_effect(() => styles = $.set_style(div, "", styles, { opacity: x }));
	$.event("click", button, function click() {
		return x = 2;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
