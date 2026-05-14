import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<img alt="" class="svelte-1qciquw"/>`);
var root = $.from_html(`<div class="icon-slot svelte-1qciquw"><!></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	var node = $.child(div);
	$.slot(node, $$props, "icon", {}, ($$anchor) => {
		var img = root_1();
		$.append($$anchor, img);
	});
	$.reset(div);
	$.append($$anchor, div);
}
