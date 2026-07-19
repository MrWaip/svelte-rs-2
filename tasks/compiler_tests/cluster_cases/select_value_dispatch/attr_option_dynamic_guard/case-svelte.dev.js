App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>Dog</option></select> <button>swap</button>`, 1), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag($.state("dog"), "v");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	var button = $.sibling(select, 2);
	$.template_effect(() => {
		if (option_value !== (option_value = $.get(v))) {
			option.value = (option.__value = $.get(v)) ?? "";
		}
	});
	$.delegated("click", button, function click() {
		return $.set(v, "cat");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
