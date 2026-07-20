import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<span>static</span>`, 1);
var option_content_1 = $.from_html(`<span> </span>`, 1);
var root = $.from_html(`<select><option><!></option></select> <select><option><!></option></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let x = $.state("hi");
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment_1 = option_content();
		$.append(anchor, fragment_1);
	});
	$.reset(select);
	var select_1 = $.sibling(select, 2);
	var option_1 = $.child(select_1);
	$.customizable_select(option_1, () => {
		var anchor_1 = $.child(option_1);
		var fragment_2 = option_content_1();
		var span = $.first_child(fragment_2);
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(x)));
		$.append(anchor_1, fragment_2);
	});
	$.reset(select_1);
	var button = $.sibling(select_1, 2);
	$.delegated("click", button, () => $.set(x, "bye"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
