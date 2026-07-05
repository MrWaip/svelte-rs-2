import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let w = 100;
	let h = 100;
	$$renderer.push(`<svg><line${$.attr("x1", 0)}${$.attr("y1", 0)}${$.attr("x2", w)}${$.attr("y2", h)}></line><rect${$.attr("width", w)}${$.attr("height", h)}></rect></svg>`);
}
