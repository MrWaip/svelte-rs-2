import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<select><option>Two</option></select>`);
export default function App($$anchor) {
	var select = root();
	var option = $.child(select);
	option.value = option.__value = "b";
	$.reset(select);
	$.append($$anchor, select);
}
