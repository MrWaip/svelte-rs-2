import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const id = $.props_id($$renderer);
	$$renderer.push(`<div${$.attr("id", id)}>hello</div>`);
}
