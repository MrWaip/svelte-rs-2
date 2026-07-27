import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let group = [];
	$$renderer.push(`<input type="radio"${$.attr("checked", group === "x", true)} value="x"/>`);
}
