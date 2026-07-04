App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta charset="utf-8"/> <meta name="description" content="A page"/> <link rel="stylesheet" href="/styles.css"/>`, 1), App[$.FILENAME], [
	[7, 1],
	[9, 1],
	[10, 1]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let section = $.tag($.state("Dashboard"), "section");
	$.set(section, "Settings");
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root();
		$.next(4);
		$.deferred_template_effect(() => {
			$.document.title = `App - ${$.get(section) ?? ""}`;
		});
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
