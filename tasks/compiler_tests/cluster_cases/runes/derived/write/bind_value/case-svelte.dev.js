App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input/> <button>s</button> `, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let s = $.tag($.state(0), "s");
	let d = $.tag($.derived(() => $.get(s) * 2), "d");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	var text = $.sibling(button);
	$.template_effect(() => $.set_text(text, ` ${$.get(d) ?? ""}`));
	$.bind_value(input, function get() {
		return $.get(d);
	}, function set($$value) {
		$.set(d, $$value);
	});
	$.delegated("click", button, function click() {
		return $.update(s);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
