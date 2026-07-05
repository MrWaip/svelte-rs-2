import * as $ from "svelte/internal/client";
var root = $.from_html(`<meta name="description" content="test"/>`);
var root_1 = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let count = $.state(0);
	function handleScroll() {
		$.update(count);
	}
	var div = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root();
		$.deferred_template_effect(() => {
			$.document.title = `Count: ${$.get(count) ?? ""}`;
		});
		$.append($$anchor, meta);
	});
	$.event("scroll", $.window, handleScroll);
	var text = $.child(div);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""}`));
	$.append($$anchor, div);
}
