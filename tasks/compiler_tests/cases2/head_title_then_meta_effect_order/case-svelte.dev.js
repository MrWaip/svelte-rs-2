App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="og:title"/>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.template_effect(() => $.set_attribute(meta, "content", $$props.title));
		$.deferred_template_effect(() => {
			$.document.title = $$props.title ?? "";
		});
		$.append($$anchor, meta);
	});
	return $.pop($$exports);
}
