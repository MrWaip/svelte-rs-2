import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root_1 = $.add_locations($.from_html(`<span>fallback</span>`), App[$.FILENAME], [[3, 1]]);
var root = $.add_locations($.from_html(`<p>before</p> <!> <p>after</p>`, 1), App[$.FILENAME], [[1, 0], [5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	$.slot(node, $$props, "footer", {}, ($$anchor) => {
		var span = root_1();
		$.append($$anchor, span);
	});
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
