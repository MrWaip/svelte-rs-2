import * as $ from "svelte/internal/client";
var option_content = $.from_html(`<span> </span>`, 1);
var root = $.from_html(`<select><option><!></option></select> <button>x</button>`, 1);
export default function App($$anchor) {
	let v = $.state("a");
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
	$.delegated("click", button, () => $.set(v, "b"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
