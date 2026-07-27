import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let f = 0;
	async function go() {
		// svelte-ignore await_reactivity_loss
		[f = false || await Promise.resolve(6)] = [];
	}
	$$renderer.push(`<button>${$.escape(f)}</button>`);
}
