import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<y>fallback content</y>`);
var root_1 = $.from_html(`<x class="svelte-1schprl"></x> <!> <z class="svelte-1schprl">this should be green if the slot fallback is not rendered</z>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var y = root();
		$.append($$anchor, y);
	});
	$.next(2);
	$.append($$anchor, fragment);
}
