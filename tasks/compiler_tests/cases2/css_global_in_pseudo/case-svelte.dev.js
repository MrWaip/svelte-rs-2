App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p class="svelte-1rk0dqc">content</p> <p class="bar svelte-1rk0dqc">bar</p> <div class="svelte-1rk0dqc">box</div>`, 1), App[$.FILENAME], [
	[12, 0],
	[13, 0],
	[14, 0]
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
