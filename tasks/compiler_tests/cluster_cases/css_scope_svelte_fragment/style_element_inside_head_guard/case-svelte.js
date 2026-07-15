import * as $ from "svelte/internal/client";
var root = $.from_html(`<style>body { background: lightblue; }</style>`);
var root_1 = $.from_html(`<h1 class="svelte-xildax">hi</h1>`);
export default function App($$anchor) {
	var h1 = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var style = root();
		$.append($$anchor, style);
	});
	$.append($$anchor, h1);
}
