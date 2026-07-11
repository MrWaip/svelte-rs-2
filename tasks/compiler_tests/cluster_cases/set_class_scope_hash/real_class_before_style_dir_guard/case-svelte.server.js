import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div class="foo svelte-1ghvvfz"${$.attr_style("", { color: "red" })}>a</div>`);
}
