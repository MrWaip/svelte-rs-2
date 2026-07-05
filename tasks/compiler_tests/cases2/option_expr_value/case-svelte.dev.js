App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>pick</button> <select><option>A</option></select>`, 1), App[$.FILENAME], [[6, 0], [
	7,
	0,
	[[8, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state("a"), "value");
	function pick() {
		$.set(value, "b");
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var select = $.sibling(button, 2);
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	$.template_effect(() => {
		if (option_value !== (option_value = $.get(value))) {
			option.value = (option.__value = $.get(value)) ?? "";
		}
	});
	$.delegated("click", button, pick);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
