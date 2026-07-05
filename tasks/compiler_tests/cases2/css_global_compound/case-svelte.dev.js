App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="a svelte-1jl4a2j"><div class="b"><div class="c">compound</div></div></div> <section class="svelte-1jl4a2j"><strong>bare global</strong></section> <h1 class="title">title</h1>`, 1), App[$.FILENAME], [
	[
		15,
		0,
		[[
			16,
			1,
			[[17, 2]]
		]]
	],
	[
		21,
		0,
		[[22, 1]]
	],
	[25, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(4);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
