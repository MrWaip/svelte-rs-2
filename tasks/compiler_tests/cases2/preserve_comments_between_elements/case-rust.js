import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>before</p> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root();
	var p = $.sibling($.first_child(fragment), 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `after ${$$props.value ?? ""}`));
	$.append($$anchor, fragment);
}
