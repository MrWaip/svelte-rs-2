import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let count = 0;
	const cleanup = $.effect_root(() => {
		console.log("root effect:", count);
		return () => console.log("cleanup");
	});
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
}
