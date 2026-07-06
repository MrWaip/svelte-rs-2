import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attr_style("", {
		width: "18px",
		height: "18px"
	})}></div>`);
}
