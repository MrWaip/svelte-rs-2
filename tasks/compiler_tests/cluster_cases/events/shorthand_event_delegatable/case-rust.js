import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let onkeydown = $.prop($$props, "onkeydown", 8, undefined);
	var div = root();
	$.delegated("keydown", div, function(...$$args) {
		onkeydown()?.apply(this, $$args);
	});
	$.append($$anchor, div);
}
$.delegate(["keydown"]);
