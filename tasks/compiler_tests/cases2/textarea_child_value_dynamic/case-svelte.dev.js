App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<textarea></textarea>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = "hello";
	var $$exports = { ...$.legacy_api() };
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.set_value(textarea, value);
	$.append($$anchor, textarea);
	return $.pop($$exports);
}
