App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="card svelte-1u8u4ji"><h2 class="title svelte-1u8u4ji">inside</h2></div> <section class="panel svelte-1u8u4ji"><h3 class="label svelte-1u8u4ji">implicit</h3></section> <h2 class="title">outside</h2> <h3 class="label">outside implicit</h3>`, 1), App[$.FILENAME], [
	[
		10,
		0,
		[[11, 4]]
	],
	[
		14,
		0,
		[[15, 4]]
	],
	[18, 0],
	[19, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(6);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
