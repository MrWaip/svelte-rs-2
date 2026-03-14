import * as $ from "svelte/internal/client";
const greeting = ($$anchor, name = $.noop) => {
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `Hello ${name() ?? ""}`));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor, $$props) {
	let title = $.prop($$props, "title", 3, "world");
	let message = "hello";
	var fragment = root();
	var node = $.first_child(fragment);
	greeting(node, () => message);
	var node_1 = $.sibling(node, 2);
	greeting(node_1, title);
	$.append($$anchor, fragment);
}
