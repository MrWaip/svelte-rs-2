import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	var html;
	var $$promises = $.run([async () => html = await Promise.resolve("<b>hi</b>")]);
	var div = root();
	var node = $.child(div);
	$.async(node, [$$promises[0]], void 0, (node) => {
		$.html(node, () => html);
	});
	$.reset(div);
	$.append($$anchor, div);
}
