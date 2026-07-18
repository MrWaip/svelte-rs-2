import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>Dog</option><option>Cat</option></select> <button>swap</button>`, 1);
export default function App($$anchor) {
	let v = $.state("dog");
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	option.value = option.__value = "dog";
	var option_1 = $.sibling(option);
	option_1.value = option_1.__value = "cat";
	$.reset(select);
	var select_value;
	$.init_select(select);
	var button = $.sibling(select, 2);
	$.template_effect(() => {
		if (select_value !== (select_value = $.get(v))) {
			select.value = (select.__value = $.get(v)) ?? "", $.select_option(select, $.get(v));
		}
	});
	$.delegated("click", button, () => $.set(v, "cat"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
