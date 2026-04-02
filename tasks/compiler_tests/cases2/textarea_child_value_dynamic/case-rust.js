import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor) {
	let value = "hello";
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.set_value(textarea, value);
	$.append($$anchor, textarea);
}
