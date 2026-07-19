import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<y>fallback content</y>`), App[$.FILENAME], [[4, 1]]);
var root_1 = $.add_locations($.from_html(`<x class="svelte-1schprl"></x> <!> <z class="svelte-1schprl">this should be green if the slot fallback is not rendered</z>`, 1), App[$.FILENAME], [[1, 0], [7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var y = root();
		$.append($$anchor, y);
	});
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
