import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-omh8kw"><span>icon</span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
