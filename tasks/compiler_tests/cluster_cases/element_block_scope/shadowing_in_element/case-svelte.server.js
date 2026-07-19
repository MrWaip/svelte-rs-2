import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	$$renderer.push(`<button>${$.escape(x)}</button> `);
	{
		const x = "inner";
		$$renderer.push(`<div><b>inner</b></div>`);
	}
}
