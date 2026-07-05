App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>+</button> <select><option>A</option></select>`, 1), App[$.FILENAME], [[6, 0], [
	7,
	0,
	[[8, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(1), "count");
	function inc() {
		$.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var select = $.sibling(button, 2);
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	$.template_effect(() => {
		if (option_value !== (option_value = `x-${$.get(count)}`)) {
			option.value = option.__value = `x-${$.get(count)}`;
		}
	});
	$.delegated("click", button, inc);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
