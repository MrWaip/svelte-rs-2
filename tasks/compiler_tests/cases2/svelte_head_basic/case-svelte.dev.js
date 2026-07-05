App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="description" content="A great app"/> <link rel="icon" href="/favicon.ico"/>`, 1), App[$.FILENAME], [[2, 1], [3, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root();
		$.next(2);
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
