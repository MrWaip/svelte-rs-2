import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<span> </span>`, 1);
var root = $.from_html(`<select><option><!></option><option>B</option></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let v = $.state("x");
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
	option.value = option.__value = "a";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "b";
	$.reset(select);
	var button = $.sibling(select, 2);
	$.delegated("click", button, () => $.set(v, "y"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
