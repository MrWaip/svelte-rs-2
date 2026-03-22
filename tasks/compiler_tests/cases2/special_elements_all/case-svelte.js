import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<meta name="viewport" content="width=device-width"/>`);
var root = $.from_html(`<div><p> </p></div>`);
export default function App($$anchor) {
	let count = $.state(0);
	function handleEvent() {
		$.update(count);
	}
	function action(node) {
		return { destroy() {} };
	}
	var div = root();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.deferred_template_effect(() => {
			$.document.title = `App: ${$.get(count) ?? ""}`;
		});
		$.append($$anchor, meta);
	});
	$.event("scroll", $.window, handleEvent);
	$.event("visibilitychange", $.document, handleEvent);
	$.event("mouseenter", $.document.body, handleEvent);
	$.action($.document.body, ($$node) => action?.($$node));
	var p = $.child(div);
	var text = $.child(p);
	$.reset(p);
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Count: ${$.get(count) ?? ""}`));
	$.append($$anchor, div);
}
