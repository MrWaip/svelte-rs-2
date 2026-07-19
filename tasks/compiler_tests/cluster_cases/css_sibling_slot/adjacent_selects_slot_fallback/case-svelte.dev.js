import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span class="svelte-142nm4m">Hello</span>`), App[$.FILENAME], [[3, 2]]);
var root_1 = $.add_locations($.from_html(`<h1 class="svelte-142nm4m">test</h1> <!>`, 1), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var span = root();
		$.append($$anchor, span);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
