import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor, $$props) {
	let value = $.prop($$props, "value", 12, "");
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.bind_value(textarea, value);
	$.append($$anchor, textarea);
}
