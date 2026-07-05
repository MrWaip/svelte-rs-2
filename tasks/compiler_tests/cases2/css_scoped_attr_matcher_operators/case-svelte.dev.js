App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div data-tags="card active" class="svelte-v4glr4">class</div> <div data-lang="en-US" class="svelte-v4glr4">lang</div> <div data-url="https://example.com" class="svelte-v4glr4">href</div> <span data-tags="inactive">no class</span> <div data-lang="bengali">no lang</div> <div data-url="http://sample.org">no href</div>`, 1), App[$.FILENAME], [
	[23, 0],
	[24, 0],
	[25, 0],
	[26, 0],
	[27, 0],
	[28, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(10);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
