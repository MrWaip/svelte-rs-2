import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { width = "10px" } = $$props;
	$$renderer.push(`<div${$.attr_style("color: red", { width })}></div>`);
}
