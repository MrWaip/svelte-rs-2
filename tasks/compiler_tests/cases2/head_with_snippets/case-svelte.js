import * as $ from "svelte/internal/client";
const badge = ($$anchor, text = $.noop) => {
	var span = root_2();
	var text_1 = $.child(span, true);
	$.reset(span);
	$.template_effect(() => $.set_text(text_1, text()));
	$.append($$anchor, span);
};
var root_1 = $.from_html(`<meta name="description" content="test"/>`);
var root_2 = $.from_html(`<span class="badge"> </span>`);
var root = $.from_html(`<div><p></p> <!></div>`);
export default function App($$anchor) {
	let title = "hello";
	var div = root();
	$.head("q2w0q4", ($$anchor) => {
		var meta = root_1();
		$.append($$anchor, meta);
	});
	var p = $.child(div);
	p.textContent = "hello";
	var node = $.sibling(p, 2);
	badge(node, () => "new");
	$.reset(div);
	$.append($$anchor, div);
}
