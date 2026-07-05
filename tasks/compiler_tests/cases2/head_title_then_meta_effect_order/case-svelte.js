import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta name="og:title"/>`);
export default function App($$anchor, $$props) {
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.template_effect(() => $.set_attribute(meta, "content", $$props.title));
		$.deferred_template_effect(() => {
			$.document.title = $$props.title ?? "";
		});
		$.append($$anchor, meta);
	});
}
