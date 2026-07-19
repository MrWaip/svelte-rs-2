import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="svelte-142nm4m">Hello</span>`);
var root_1 = $.from_html(`<h1 class="svelte-142nm4m">test</h1> <!>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var span = root();
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
}
