App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div data-state="on" class="svelte-85638w">exact</div> <div data-state="On" class="svelte-85638w">insensitive</div> <div data-state="off">off</div> <button type="button" aria-label="run" class="svelte-85638w">button</button>`, 1), App[$.FILENAME], [
	[19, 0],
	[20, 0],
	[21, 0],
	[22, 0]
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
