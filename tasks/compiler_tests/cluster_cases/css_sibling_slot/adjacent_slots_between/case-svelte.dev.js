import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1 class="svelte-142nm4m">test</h1> <!> <!> <!> <span class="svelte-142nm4m">Hello</span>`, 1), App[$.FILENAME], [[1, 0], [5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "a", {}, null);
	var node_1 = $.sibling(node, 2);
	$.slot(node_1, $$props, "b", {}, null);
	var node_2 = $.sibling(node_1, 2);
	$.slot(node_2, $$props, "c", {}, null);
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
