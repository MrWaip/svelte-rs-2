import * as $ from "svelte/internal/client";
const a = ($$anchor) => {
	var fragment = root_1();
	var node = $.first_child(fragment);
	b(node);
	var div = $.sibling(node, 2);
	var node_1 = $.child(div);
	b(node_1);
	$.reset(div);
	$.append($$anchor, fragment);
};
const b = ($$anchor) => {
	var fragment_1 = root_2();
	var node_2 = $.first_child(fragment_1);
	a(node_2);
	var div_1 = $.sibling(node_2, 2);
	var node_3 = $.child(div_1);
	a(node_3);
	$.reset(div_1);
	$.append($$anchor, fragment_1);
};
var root_1 = $.from_html(`<!> <div class="svelte-ile0r9"><!></div>`, 1);
var root_2 = $.from_html(`<!> <div class="svelte-ile0r9"><!></div>`, 1);
export default function App($$anchor) {}
