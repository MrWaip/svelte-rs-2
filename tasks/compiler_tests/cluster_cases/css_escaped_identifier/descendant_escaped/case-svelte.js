import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div id="1" class="svelte-y5u8ao"><span class="svelte-y5u8ao"></span></div>`);
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
