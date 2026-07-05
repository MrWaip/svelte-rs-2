import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<label>b <input type="checkbox"/></label>`), App[$.FILENAME], [[
	12,
	1,
	[[12, 10]]
]]);
var root_1 = $.add_locations($.from_html(`<button> </button> <label>a <input type="checkbox"/></label> <!> <label> <input value="just here, so b is not the last input"/></label>`, 1), App[$.FILENAME], [
	[6, 0],
	[
		10,
		0,
		[[10, 9]]
	],
	[
		14,
		0,
		[[14, 37]]
	]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let test = $.prop($$props, "test", 28, () => []);
	let hidden = $.tag($.mutable_source(false), "hidden");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var text = $.child(button);
	$.reset(button);
	var label = $.sibling(button, 2);
	var input = $.sibling($.child(label));
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	$.reset(label);
	var node = $.sibling(label, 2);
	{
		var consequent = ($$anchor) => {
			var label_1 = root();
			var input_1 = $.sibling($.child(label_1));
			$.remove_input_defaults(input_1);
			input_1.value = input_1.__value = "b";
			$.reset(label_1);
			$.bind_group(binding_group, [], input_1, function get() {
				return test();
			}, function set($$value) {
				test($$value);
			});
			$.append($$anchor, label_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (!$.get(hidden)) $$render(consequent);
		}), "if", App, 11, 0);
	}
	var label_2 = $.sibling(node, 2);
	var text_1 = $.child(label_2);
	var input_2 = $.sibling(text_1);
	$.reset(label_2);
	$.template_effect(() => {
		$.set_text(text, `${$.get(hidden) ? "show" : "hide"} b`);
		$.set_text(text_1, `c${$.get(hidden) ? "show" : "hide"} b `);
	});
	$.event("click", button, function click() {
		return $.set(hidden, !$.get(hidden));
	});
	$.bind_group(binding_group, [], input, function get() {
		return test();
	}, function set($$value) {
		test($$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
