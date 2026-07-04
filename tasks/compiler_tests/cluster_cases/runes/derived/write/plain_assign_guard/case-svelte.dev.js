App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button> <button>s</button> `, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let s = $.tag($.state(0), "s");
	let d = $.tag($.derived(() => $.get(s) * 2), "d");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var text = $.sibling(button_1);
	$.template_effect(() => $.set_text(text, ` ${$.get(d) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.set(d, 1);
	});
	$.delegated("click", button_1, function click_1() {
		return $.update(s);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
