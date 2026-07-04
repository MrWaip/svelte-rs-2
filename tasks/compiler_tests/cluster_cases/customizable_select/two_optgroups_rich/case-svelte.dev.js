App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var option_content = $.add_locations($.from_html(`<span> </span>`, 1), App[$.FILENAME], [[9, 20]]);
var optgroup_content = $.add_locations($.from_html(`<span class="fh"> </span> <option><!></option>`, 1), App[$.FILENAME], [[8, 2], [9, 2]]);
var option_content_1 = $.add_locations($.from_html(`<em> </em>`, 1), App[$.FILENAME], [[13, 20]]);
var optgroup_content_1 = $.add_locations($.from_html(`<em class="vh"> </em> <option><!></option>`, 1), App[$.FILENAME], [[12, 2], [13, 2]]);
var root = $.add_locations($.from_html(`<select><optgroup label="Fruits"><!></optgroup><optgroup label="Vegs"><!></optgroup></select> <button>x</button>`, 1), App[$.FILENAME], [[
	6,
	0,
	[[7, 1], [11, 1]]
], [17, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let fruit = $.tag($.state("apple"), "fruit");
	let veggie = $.tag($.state("carrot"), "veggie");
	var $$exports = { ...$.legacy_api() };
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
			$.template_effect(() => $.set_text(text_1, $.get(fruit)));
			$.append(anchor_1, fragment_2);
		});
		option.value = option.__value = "a";
		$.template_effect(() => $.set_text(text, $.get(fruit)));
		$.append(anchor, fragment_1);
	});
	var optgroup_1 = $.sibling(optgroup);
	$.customizable_select(optgroup_1, () => {
		var anchor_2 = $.child(optgroup_1);
		var fragment_3 = optgroup_content_1();
		var em = $.first_child(fragment_3);
		var text_2 = $.child(em, true);
		$.reset(em);
		var option_1 = $.sibling(em, 2);
		$.customizable_select(option_1, () => {
			var anchor_3 = $.child(option_1);
			var fragment_4 = option_content_1();
			var em_1 = $.first_child(fragment_4);
			var text_3 = $.child(em_1, true);
			$.reset(em_1);
			$.template_effect(() => $.set_text(text_3, $.get(veggie)));
			$.append(anchor_3, fragment_4);
		});
		option_1.value = option_1.__value = "c";
		$.template_effect(() => $.set_text(text_2, $.get(veggie)));
		$.append(anchor_2, fragment_3);
	});
	$.reset(select);
	var button = $.sibling(select, 2);
	$.delegated("click", button, function click() {
		$.set(fruit, "orange");
		$.set(veggie, "broccoli");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
