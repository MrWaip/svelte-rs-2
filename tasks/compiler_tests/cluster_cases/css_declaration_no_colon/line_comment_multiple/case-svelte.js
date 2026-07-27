import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="row svelte-161ykp3">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
