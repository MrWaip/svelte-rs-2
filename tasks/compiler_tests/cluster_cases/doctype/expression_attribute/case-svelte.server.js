import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let kind = "html";
	$$renderer.push(`<button>toggle</button> <!doctype${$.attr("html", kind)}/>`);
}
