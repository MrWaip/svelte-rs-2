import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let group = [];
	$$renderer.push(`<input type="checkbox"${$.attr("checked", group.includes(true), true)} value=""/>`);
}
