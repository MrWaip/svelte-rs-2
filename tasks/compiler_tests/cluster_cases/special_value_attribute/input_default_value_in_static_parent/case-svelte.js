import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x"><input value="x"/></div>`);
export default function App($$anchor) {
	var div = root();
	var input = $.child(div);
	$.set_default_value(input, "y");
	$.reset(div);
	$.append($$anchor, div);
}
