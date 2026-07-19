import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>Dog</option></select> <button>swap</button>`, 1);
export default function App($$anchor) {
	let v = $.state("dog");
	var fragment = root();
	var select = $.first_child(fragment);
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	var button = $.sibling(select, 2);
	$.template_effect(() => {
		if (option_value !== (option_value = $.get(v))) {
			option.value = (option.__value = $.get(v)) ?? "";
		}
	});
	$.delegated("click", button, () => $.set(v, "cat"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
