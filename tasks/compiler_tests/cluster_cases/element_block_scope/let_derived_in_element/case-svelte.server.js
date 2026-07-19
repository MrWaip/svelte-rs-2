import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		let dt = $.derived(() => 1 + 2);
		$$renderer.push(`<div>number</div>`);
	}
}
