import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<textarea></textarea> <textarea value="hello"></textarea> <textarea></textarea>`, 1), App[$.FILENAME], [
	[5, 0],
	[6, 0],
	[7, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	var $$exports = { ...$.legacy_api() };
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
	return $.pop($$exports);
}
