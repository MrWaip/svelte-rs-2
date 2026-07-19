import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<a class="svelte-190fkm3"><b>b</b></a>`);
export default function App($$anchor) {
	var a = root();
	$.append($$anchor, a);
}
