App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var option_content = $.add_locations($.from_html(`<span> </span>`, 1), App[$.FILENAME], [[6, 19]]);
var root = $.add_locations($.from_html(`<select><option><!></option></select> <button>x</button>`, 1), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let v = $.tag($.state("a"), "v");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment_1 = option_content();
		var span = $.first_child(fragment_1);
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append(anchor, fragment_1);
	});
	var option_value = {};
	$.reset(select);
	var button = $.sibling(select, 2);
	$.template_effect(() => {
		if (option_value !== (option_value = $.get(v))) {
			option.value = (option.__value = $.get(v)) ?? "";
		}
	});
	$.delegated("click", button, function click() {
		return $.set(v, "b");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
