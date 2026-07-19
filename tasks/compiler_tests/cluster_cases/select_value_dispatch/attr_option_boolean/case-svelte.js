import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>a</option><option>b</option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	option.value = option.__value = true;
	$.next();
	$.reset(select);
	$.append($$anchor, select);
}
