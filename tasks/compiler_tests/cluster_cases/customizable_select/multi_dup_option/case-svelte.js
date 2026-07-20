import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<b> </b>`, 1);
var root = $.from_html(`<select><option><!></option></select> <select><option><!></option></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let x = $.state("hi");
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment_1 = option_content();
		var b = $.first_child(fragment_1);
		var text = $.child(b, true);
		$.reset(b);
		$.template_effect(() => $.set_text(text, $.get(x)));
		$.append(anchor, fragment_1);
	});
	$.reset(select);
	var select_1 = $.sibling(select, 2);
	var option_1 = $.child(select_1);
	$.customizable_select(option_1, () => {
		var anchor_1 = $.child(option_1);
		var fragment_2 = option_content();
		var b_1 = $.first_child(fragment_2);
		var text_1 = $.child(b_1, true);
		$.reset(b_1);
		$.template_effect(() => $.set_text(text_1, $.get(x)));
		$.append(anchor_1, fragment_2);
	});
	$.reset(select_1);
	var button = $.sibling(select_1, 2);
	$.delegated("click", button, () => $.set(x, "bye"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
