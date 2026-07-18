import * as $ from "svelte/internal/client";
var root = $.from_html(`<input disabled=""/>`);
export default function App($$anchor) {
	var iNPUT = root();
	$.append($$anchor, iNPUT);
}
