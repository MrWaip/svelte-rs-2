import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="svelte-1j0crw7">Span 1</span>`);
var root_1 = $.from_html(`<span class="svelte-1j0crw7">Span 2</span>`);
var root_2 = $.from_html(`<h1 class="svelte-1j0crw7">Heading 1</h1> <!> <!> <p class="svelte-1j0crw7">Paragraph 2</p>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_2();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var span = root();
		$.append($$anchor, span);
	});
	var node_1 = $.sibling(node, 2);
	$.slot(node_1, $$props, "default", {}, ($$anchor) => {
		var span_1 = root_1();
		$.append($$anchor, span_1);
	});
	$.next(2);
	$.append($$anchor, fragment);
}
