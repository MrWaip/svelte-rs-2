import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span>fallback</span>`);
var root_1 = $.from_html(`<p>before</p> <!> <p>after</p>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "footer", {}, ($$anchor) => {
		var span = root();
		$.append($$anchor, span);
	});
	$.next(2);
	$.append($$anchor, fragment);
}
