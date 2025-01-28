import * as $ from "svelte/internal/client";
var root = $.template(`<div>_</div> `, 1);
export default function App($$anchor) {
	let title = 10;
	let title2 = 12;
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.sibling(div, 1, true);
	$.template_effect(() => {
		$.set_attribute(div, "attr", $.update(title));
		$.set_text(text, $.update_pre(title2, -1));
	});
	$.append($$anchor, fragment);
}
