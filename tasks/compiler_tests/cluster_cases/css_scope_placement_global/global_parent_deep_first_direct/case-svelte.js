import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="a svelte-1oo9t8i"><i class="b svelte-1oo9t8i">b</i></span>`);
export default function App($$anchor) {
	var span = root();
	$.append($$anchor, span);
}
