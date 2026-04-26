import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	let name = $.prop($$props, "name", 8);
	let typed = $.prop($$props, "typed", 8, null);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, name());
		$.set_text(text_1, typed());
	});
	$.append($$anchor, fragment);
}
