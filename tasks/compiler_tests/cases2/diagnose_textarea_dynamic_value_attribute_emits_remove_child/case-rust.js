import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor, $$props) {
	let v = $.prop($$props, "v", 3, "");
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.template_effect(() => $.set_value(textarea, v()));
	$.append($$anchor, textarea);
}
