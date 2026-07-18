App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/> <button>swap</button>`, 1), App[$.FILENAME], [[6, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let group = $.tag($.state($.proxy([])), "group");
	let v = $.tag($.state("x"), "v");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_value;
	var button = $.sibling(input, 2);
	$.template_effect(() => {
		if (input_value !== (input_value = $.get(v))) {
			input.value = (input.__value = $.get(v)) ?? "";
		}
	});
	$.bind_group(binding_group, [], input, () => {
		$.get(v);
		return $.get(group);
	}, function set($$value) {
		$.set(group, $$value);
	});
	$.delegated("click", button, function click() {
		return $.set(v, "y");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
