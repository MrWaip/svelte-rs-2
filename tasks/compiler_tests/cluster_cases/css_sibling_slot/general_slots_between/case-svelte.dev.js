import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span class="svelte-1j0crw7">Span 1</span>`), App[$.FILENAME], [[3, 2]]);
var root_1 = $.add_locations($.from_html(`<span class="svelte-1j0crw7">Span 2</span>`), App[$.FILENAME], [[6, 2]]);
var root_2 = $.add_locations($.from_html(`<h1 class="svelte-1j0crw7">Heading 1</h1> <!> <!> <p class="svelte-1j0crw7">Paragraph 2</p>`, 1), App[$.FILENAME], [[1, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
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
	return $.pop($$exports);
}
