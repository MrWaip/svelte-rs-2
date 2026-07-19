import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1 class="svelte-142nm4m">test</h1> <!> <!> <!> <span class="svelte-142nm4m">Hello</span>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "a", {}, null);
	var node_1 = $.sibling(node, 2);
	$.slot(node_1, $$props, "b", {}, null);
	var node_2 = $.sibling(node_1, 2);
	$.slot(node_2, $$props, "c", {}, null);
	$.next(2);
	$.append($$anchor, fragment);
}
