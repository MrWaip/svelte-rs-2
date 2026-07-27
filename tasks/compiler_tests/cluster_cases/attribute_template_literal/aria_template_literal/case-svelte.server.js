import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attr("aria-level", `abc`)}></div> <div aria-level="false"></div> <div${$.attr("title", `x`)}></div>`);
}
