App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="description" content="test"/>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let pageTitle = $.tag($.state("Home"), "pageTitle");
	let section = $.tag($.state("Dashboard"), "section");
	$.set(pageTitle, "Other");
	$.set(section, "Settings");
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.deferred_template_effect(() => {
			$.document.title = `Static - ${$.get(pageTitle) ?? ""} - App ${$.get(section) ?? ""}`;
		});
		$.append($$anchor, meta);
	});
	return $.pop($$exports);
}
