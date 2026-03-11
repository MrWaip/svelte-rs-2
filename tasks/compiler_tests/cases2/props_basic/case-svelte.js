import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	let y = $.prop($$props, "y", 3, 10);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, $$props.x);
		$.set_text(text_1, y());
	});
	$.append($$anchor, fragment);
}
