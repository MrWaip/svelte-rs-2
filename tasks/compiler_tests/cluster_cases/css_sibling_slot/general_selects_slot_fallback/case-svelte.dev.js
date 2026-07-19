import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="svelte-11j9i8n">Paragraph 2</p>`), App[$.FILENAME], [[5, 2]]);
var root_1 = $.add_locations($.from_html(`<h1 class="svelte-11j9i8n">Heading 1</h1> <span>Span 1</span> <span>Span 2</span> <!>`, 1), App[$.FILENAME], [
	[1, 0],
	[2, 0],
	[3, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 6);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
