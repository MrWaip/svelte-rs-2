App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/> <button>rotate</button>`, 1), App[$.FILENAME], [[10, 0], [11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let initial = $.prop($$props, "initial", 3, "a");
	let selected = $.tag($.state(null), "selected");
	let dyn_val = $.tag($.state($.proxy(initial())), "dyn_val");
	function rotate() {
		$.set(dyn_val, $.get(dyn_val) + "!");
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_value;
	var button = $.sibling(input, 2);
	$.template_effect(() => {
		if (input_value !== (input_value = `item-${$.get(dyn_val)}`)) {
			input.value = input.__value = `item-${$.get(dyn_val)}`;
		}
	});
	$.bind_group(binding_group, [], input, () => {
		`item-${$.get(dyn_val)}`;
		return $.get(selected);
	}, function set($$value) {
		$.set(selected, $$value);
	});
	$.delegated("click", button, rotate);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
