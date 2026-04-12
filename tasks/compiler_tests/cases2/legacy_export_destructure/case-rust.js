import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let { x: foo, z: [bar] } = {
		x: "a",
		z: ["b"]
	};
	var p = root();
	p.textContent = `${foo ?? ""}${bar ?? ""}`;
	$.append($$anchor, p);
}
