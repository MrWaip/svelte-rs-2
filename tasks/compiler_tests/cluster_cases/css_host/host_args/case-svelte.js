import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1 class="svelte-wrbajf">h</h1><div><span>s</span></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}
