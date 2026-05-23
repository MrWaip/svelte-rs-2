import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><!> </div>`);
export default function App($$anchor, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $.prop($$props, "x", 8);
	var div = root();
	$.set_class(div, 1, "", null, {}, { "before-content": $$slots.beforeContent });
	var node = $.child(div);
	$.slot(node, $$props, "beforeContent", {}, null);
	var text = $.sibling(node);
	$.reset(div);
	$.template_effect(() => $.set_text(text, ` ${x() ?? ""}`));
	$.append($$anchor, div);
}
