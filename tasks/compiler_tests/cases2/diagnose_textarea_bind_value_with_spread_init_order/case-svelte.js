import * as $ from "svelte/internal/client";
var root = $.from_html(`<textarea></textarea>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 15, ""), extra = $.prop($$props, "extra", 19, () => ({}));
	var textarea = root();
	$.remove_textarea_child(textarea);
	$.attribute_effect(textarea, () => ({ ...extra() }));
	$.bind_value(textarea, value);
	$.append($$anchor, textarea);
	$.pop();
}
