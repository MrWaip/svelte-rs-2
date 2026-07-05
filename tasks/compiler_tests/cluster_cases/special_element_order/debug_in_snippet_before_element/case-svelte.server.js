import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function row($$renderer) {
		console.log({ count });
		debugger;
		$$renderer.push(`<button>+</button>`);
	}
	row($$renderer);
}
