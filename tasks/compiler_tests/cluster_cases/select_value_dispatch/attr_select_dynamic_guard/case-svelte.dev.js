App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>Dog</option><option>Cat</option></select> <button>swap</button>`, 1), App[$.FILENAME], [[
	5,
	0,
	[[6, 1], [7, 1]]
], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag($.state("dog"), "v");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	option.value = option.__value = "dog";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "cat";
	$.reset(select);
	var select_value;
	$.init_select(select);
	var button = $.sibling(select, 2);
	$.template_effect(() => {
		if (select_value !== (select_value = $.get(v))) {
			select.value = (select.__value = $.get(v)) ?? "", $.select_option(select, $.get(v));
		}
	});
	$.delegated("click", button, function click() {
		return $.set(v, "cat");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
