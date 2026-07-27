import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`AB`, 1);
export default function App($$anchor) {
	$.next();
	var fragment = root();
	$.append($$anchor, fragment);
}
