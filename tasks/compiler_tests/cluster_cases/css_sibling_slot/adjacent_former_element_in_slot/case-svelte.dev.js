import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1 class="svelte-142nm4m">test</h1>`), App[$.FILENAME], [[2, 2]]);
var root_1 = $.add_locations($.from_html(`<!> <span class="svelte-142nm4m">Hello</span>`, 1), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var h1 = root();
		$.append($$anchor, h1);
	});
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
