import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let coords = {
		x: 0,
		y: 0
	};
	let x = $.derived(() => coords.x), y = $.derived(() => coords.y);
	$$renderer.push(`<p>${$.escape(x())},${$.escape(y())}</p>`);
}
