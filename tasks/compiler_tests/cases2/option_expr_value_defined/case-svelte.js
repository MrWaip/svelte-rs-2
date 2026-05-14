import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>+</button> <select><option>A</option></select>`, 1);
export default function App($$anchor) {
	let count = $.state(1);
	function inc() {
		$.update(count);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var select = $.sibling(button, 2);
	var option = $.child(select);
	var option_value = {};
	$.reset(select);
	$.template_effect(() => {
		if (option_value !== (option_value = `x-${$.get(count)}`)) {
			option.value = option.__value = `x-${$.get(count)}`;
		}
	});
	$.delegated("click", button, inc);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
