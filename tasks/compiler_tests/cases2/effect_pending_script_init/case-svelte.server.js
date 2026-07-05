import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pending = $.derived(() => 0);
	$$renderer.push(`<p>${$.escape(pending())}</p>`);
}
