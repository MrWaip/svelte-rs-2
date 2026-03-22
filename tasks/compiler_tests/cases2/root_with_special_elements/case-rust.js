import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let count = $.state(0);
	function handleScroll() {
		$.update(count);
	}
	$.event("scroll", $.window, handleScroll);
	var div = root();
	var text = $.child(div);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""}`));
	$.append($$anchor, div);
}
