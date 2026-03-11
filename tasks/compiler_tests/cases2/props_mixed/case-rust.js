import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let b = $.prop($$props, "b", 3, 10), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"a",
		"b",
		"c"
	]);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	var p_2 = $.sibling(p_1, 2);
	var text_2 = $.child(p_2, true);
	$.reset(p_2);
	$.template_effect(() => {
		$.set_text(text, $$props.a);
		$.set_text(text_1, b());
		$.set_text(text_2, $$props.c);
	});
	$.append($$anchor, fragment);
	$.pop();
}
