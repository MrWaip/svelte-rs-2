App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div data-role="banner" class="svelte-1k56wwr">banner</div> <button type="button" aria-label="run" class="svelte-1k56wwr">run</button> <svg viewBox="0 0 10 10" class="svelte-1k56wwr"></svg>`, 1), App[$.FILENAME], [
	[15, 0],
	[16, 0],
	[17, 0]
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
