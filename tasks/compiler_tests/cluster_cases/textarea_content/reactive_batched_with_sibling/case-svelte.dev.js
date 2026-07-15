App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button> <input/> <textarea></textarea>`, 1), App[$.FILENAME], [
	[10, 0],
	[11, 0],
	[12, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let query = $.tag($.state(""), "query");
	let name = $.tag($.state(""), "name");
	function upd() {
		$.set(query, "a");
		$.set(name, "b");
	}
	var $$exports = { ...$.legacy_api() };
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
	return $.pop($$exports);
}
$.delegate(["click"]);
