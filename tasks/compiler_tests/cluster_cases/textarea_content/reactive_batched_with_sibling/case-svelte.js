import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button> <input/> <textarea></textarea>`, 1);
export default function App($$anchor) {
	let query = $.state("");
	let name = $.state("");
	function upd() {
		$.set(query, "a");
		$.set(name, "b");
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var input = $.sibling(button, 2);
	$.remove_input_defaults(input);
	var textarea = $.sibling(input, 2);
	$.remove_textarea_child(textarea);
	$.template_effect(() => {
		$.set_value(input, $.get(name));
		$.set_value(textarea, $.get(query));
	});
	$.delegated("click", button, upd);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
