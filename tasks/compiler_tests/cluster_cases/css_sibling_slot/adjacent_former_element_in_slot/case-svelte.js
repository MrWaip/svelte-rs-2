import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1 class="svelte-142nm4m">test</h1>`);
var root_1 = $.from_html(`<!> <span class="svelte-142nm4m">Hello</span>`, 1);
export default function App($$anchor, $$props) {
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var h1 = root();
		$.append($$anchor, h1);
	});
	$.next(2);
	$.append($$anchor, fragment);
}
