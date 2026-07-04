App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="svelte-11581yn">content</p> <section class="svelte-11581yn"><span class="inner">inside</span></section>`, 1), App[$.FILENAME], [[10, 0], [
	11,
	0,
	[[12, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
