import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let outer = $.derived(() => Date.now());
	{
		let inner = $.derived(() => Date.now());
		$$renderer.push(`<div>${$.escape(inner)}</div>`);
	}
	$$renderer.push(` <p>${$.escape(outer())}</p>`);
}
