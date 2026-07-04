import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="description" content="A"/>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text("A");
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.append($$anchor, meta);
	});
	$.append($$anchor, text);
	return $.pop($$exports);
}
