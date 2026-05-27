import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<span> </span> `, 1);
var optgroup_content = $.from_html(`<span class="hdr"> </span> <option><!></option> <option>banana</option>`, 1);
var root = $.from_html(`<select><optgroup label="Fruits"><!></optgroup></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let fruit = $.state("apple");
	var fragment = root();
	var select = $.first_child(fragment);
	var optgroup = $.child(select);
	$.customizable_select(optgroup, () => {
		var anchor = $.child(optgroup);
		var fragment_1 = optgroup_content();
		var span = $.first_child(fragment_1);
		var text = $.child(span, true);
		$.reset(span);
		var option = $.sibling(span, 2);
		$.customizable_select(option, () => {
			var anchor_1 = $.child(option);
			var fragment_2 = option_content();
			var span_1 = $.first_child(fragment_2);
			var text_1 = $.child(span_1, true);
			$.reset(span_1);
			var text_2 = $.sibling(span_1);
			$.template_effect(() => {
				$.set_text(text_1, $.get(fruit));
				$.set_text(text_2, ` ${$.get(fruit) ?? ""}`);
			});
			$.append(anchor_1, fragment_2);
		});
		option.value = option.__value = "a";
		var option_1 = $.sibling(option, 2);
		option_1.value = option_1.__value = "b";
		$.template_effect(() => $.set_text(text, $.get(fruit)));
		$.append(anchor, fragment_1);
	});
	$.reset(select);
	var button = $.sibling(select, 2);
	$.delegated("click", button, () => $.set(fruit, "orange"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
