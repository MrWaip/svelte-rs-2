import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let count = $.derived(() => 0);
	let postfix = $.update(count);
	let postfix_minus = $.update(count, -1);
	let prefix = $.update_pre(count);
	let prefix_minus = $.update_pre(count, -1);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${postfix ?? ""}, ${postfix_minus ?? ""}, ${prefix ?? ""}, ${prefix_minus ?? ""}, ${$.get(count) ?? ""}`));
	$.append($$anchor, p);
}
