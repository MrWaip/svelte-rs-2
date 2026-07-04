App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="description"/>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.set_attribute(meta, "content", count);
		$.append($$anchor, meta);
	});
	return $.pop($$exports);
}
