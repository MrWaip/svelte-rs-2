import * as $ from "svelte/internal/client";
var root = $.template(`<div>_</div> `, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let title2 = $.state(12);
	$.update(title, -1);
	$.update_pre(title2);
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.sibling(div);
	$.template_effect(() => {
		$.set_attribute(div, "attr", $.update(title));
		$.set_text(text, ` ${$.update_pre(title2, -1) ?? ""}`);
	});
	$.append($$anchor, fragment);
}
