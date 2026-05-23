import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor, $$props) {
	let extra = $.prop($$props, "extra", 19, () => ({}));
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.attribute_effect(textarea, () => ({ ...extra() }));
	$.append($$anchor, textarea);
}
