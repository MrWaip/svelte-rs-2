import * as $ from "svelte/internal/client";
var root = $.from_html(`<img alt="" class="svelte-1qciquw"/>`);
var root_1 = $.from_html(`<div class="icon-slot svelte-1qciquw"><!></div>`);
export default function App($$anchor, $$props) {
	var div = root_1();
	var node = $.child(div);
	$.slot(node, $$props, "icon", {}, ($$anchor) => {
		var img = root();
		$.append($$anchor, img);
	});
	$.reset(div);
	$.append($$anchor, div);
}
