import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div title="hockey" visible=""${$.attr("expression", name)}${$.attr("description", description)}${$.attr("index", `number: ${$.stringify(idx)}`)}></div>`);
}
