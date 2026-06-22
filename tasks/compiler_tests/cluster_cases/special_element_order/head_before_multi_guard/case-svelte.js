import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="description" content="A"/>`);
var root = $.from_html(`<span>x</span> `, 1);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 8);
	var fragment = root();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.append($$anchor, meta);
	});
	var text = $.sibling($.first_child(fragment));
	$.template_effect(() => $.set_text(text, ` ${foo() ?? ""}`));
	$.append($$anchor, fragment);
}
