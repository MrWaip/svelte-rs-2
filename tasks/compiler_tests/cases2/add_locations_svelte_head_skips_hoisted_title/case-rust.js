App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root_1 = $.add_locations($.from_html(`<meta name="x" content="y"/> <link rel="canonical" href="/"/>`, 1), App[$.FILENAME], [[7, 4], [8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = $.prop($$props, "title", 3, "x");
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root_1();
		$.next(2);
		$.deferred_template_effect(() => {
			$.document.title = title() ?? "";
		});
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
