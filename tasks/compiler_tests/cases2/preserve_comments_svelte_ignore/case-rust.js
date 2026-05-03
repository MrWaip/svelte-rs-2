import * as $ from "svelte/internal/client";
var root = $.from_html(`<!-- svelte-ignore a11y_no_static_element_interactions --> <div>click</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.first_child(fragment);
	var div = $.sibling(node, 2);
	$.delegated("click", div, () => {});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
