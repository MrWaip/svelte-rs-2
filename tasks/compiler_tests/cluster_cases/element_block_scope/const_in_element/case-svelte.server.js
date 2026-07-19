import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		const x = 5;
		$$renderer.push(`<div><span>5</span></div>`);
	}
}
