import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>pick</button> <select><option>A</option><option>B</option></select>`, 1);
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
	var option_1 = $.sibling(option);
	var option_1_value = {};
	$.reset(select);
	$.template_effect(() => {
		if (option_value !== (option_value = $.get(value))) {
			option.value = (option.__value = $.get(value)) ?? "";
		}
		if (option_1_value !== (option_1_value = `prefix-${$.get(value) ?? ""}`)) {
			option_1.value = option_1.__value = `prefix-${$.get(value) ?? ""}`;
		}
	});
	$.delegated("click", button, pick);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
