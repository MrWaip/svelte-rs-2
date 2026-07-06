import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	function update() {
		a = 5;
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
