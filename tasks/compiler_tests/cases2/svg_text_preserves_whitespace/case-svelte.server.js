import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let label = "hello";
	$$renderer.push(`<svg><text${$.attr("x", 10)}${$.attr("y", 20)}>hello</text></svg>`);
}
