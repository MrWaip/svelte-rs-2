import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1 class="svelte-qww9og">h</h1>`);
export default function App($$anchor) {
	var h1 = root();
	$.append($$anchor, h1);
}
