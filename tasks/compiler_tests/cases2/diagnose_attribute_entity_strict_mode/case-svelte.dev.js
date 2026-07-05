App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div data-a="&amp;amp=q &lt; ">a</div> <div data-b="© &amp;reg=x > foo">b</div> <div data-c="&amp;ok &amp;=q">c</div>`, 1), App[$.FILENAME], [
	[1, 0],
	[2, 0],
	[3, 0]
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
