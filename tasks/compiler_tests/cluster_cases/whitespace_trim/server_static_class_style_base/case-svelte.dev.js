App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span class=" tsCompact300XSmall">x</span> <div style="--a: b; ">y</div>`, 1), App[$.FILENAME], [[1, 0], [2, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
