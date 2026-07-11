import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea> <textarea value="hello"></textarea> <textarea></textarea>`, 1);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var fragment = root();
	var textarea = $.first_child(fragment);
	$.remove_textarea_child(textarea);
	var textarea_1 = $.sibling(textarea, 2);
	var textarea_2 = $.sibling(textarea_1, 2);
	$.remove_textarea_child(textarea_2);
	$.template_effect(() => {
		$.set_value(textarea, foo());
		$.set_value(textarea_2, `a${foo() ?? ""}b`);
	});
	$.append($$anchor, fragment);
}
