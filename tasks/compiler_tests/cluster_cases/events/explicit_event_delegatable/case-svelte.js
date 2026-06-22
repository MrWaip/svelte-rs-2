import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let onclick = $.prop($$props, "onclick", 8, undefined);
	var div = root();
	$.delegated("click", div, function(...$$args) {
		onclick()?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
$.delegate(["click"]);
