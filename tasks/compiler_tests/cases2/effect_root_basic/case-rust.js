import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let count = 0;
	$.effect_root(() => {
		console.log("root effect:", count);
	});
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
}
