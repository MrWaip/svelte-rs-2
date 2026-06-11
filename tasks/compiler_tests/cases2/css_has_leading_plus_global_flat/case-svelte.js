import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-192tx82">x</div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
