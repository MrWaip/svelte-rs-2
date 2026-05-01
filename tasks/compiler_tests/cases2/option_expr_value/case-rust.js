import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>pick</button> <select><option>A</option></select>`, 1);
export default function App($$anchor) {
	let value = $.state("a");
	function pick() {
		$.set(value, "b");
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var select = $.sibling(button, 2);
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	$.template_effect(() => {
		if (option_value !== (option_value = $.get(value))) {
			option.value = (option.__value = $.get(value)) ?? "";
		}
	});
	$.delegated("click", button, pick);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
