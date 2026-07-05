App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="foo svelte-mcqa8k">foo</p> <span class="baz svelte-mcqa8k">baz</span>`, 1), App[$.FILENAME], [[11, 0], [12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
