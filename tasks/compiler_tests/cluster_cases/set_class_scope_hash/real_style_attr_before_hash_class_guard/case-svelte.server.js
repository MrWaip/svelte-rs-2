import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attr_style("margin: 0", { color: "red" })} class="svelte-1ghvvfz">a</div>`);
}
