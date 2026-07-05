App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="svelte-r1lfc1">content</p> <p class="active svelte-r1lfc1">active</p> <h2>heading</h2> <h2 class="featured">featured</h2>`, 1), App[$.FILENAME], [
	[24, 0],
	[25, 0],
	[26, 0],
	[27, 0]
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
