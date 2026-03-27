import * as $ from "svelte/internal/client";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor, $$props) {
	let tag = $.prop($$props, "tag", 3, "div");
	let title = "hello";
	var fragment = root();
	var node = $.first_child(fragment);
	$.element(node, tag, false, ($$element, $$anchor) => {
		$.set_class($$element, 0, "first");
		var text = $.text();
		text.nodeValue = "First: hello";
		$.append($$anchor, text);
	});
	var node_1 = $.sibling(node, 2);
	$.element(node_1, tag, false, ($$element, $$anchor) => {
		$.set_class($$element_1, 0, "second");
		var text_1 = $.text();
		text_1.nodeValue = "Second: hello";
		$.append($$anchor, text_1);
	});
	$.append($$anchor, fragment);
}
