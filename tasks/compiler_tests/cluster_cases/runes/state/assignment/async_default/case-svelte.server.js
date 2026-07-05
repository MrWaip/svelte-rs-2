import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let arr = [];
	async function update() {
		[a = await Promise.resolve(3)] = arr;
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
