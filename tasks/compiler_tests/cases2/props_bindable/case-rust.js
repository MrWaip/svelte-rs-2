import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.prop($$props, "count", 11, 0);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, $$props.value);
		$.set_text(text_1, count());
	});
	$.append($$anchor, fragment);
	$.pop();
}
