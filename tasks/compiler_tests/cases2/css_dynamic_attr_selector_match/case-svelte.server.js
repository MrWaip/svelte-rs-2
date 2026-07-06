import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let open = true;
	$$renderer.push(`<div${$.attr("data-state", open ? "open" : "closed")} class="svelte-1mj6a7z">inside</div> <div data-state="closed">outside</div>`);
}
