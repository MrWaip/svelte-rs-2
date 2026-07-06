import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 0;
	const min = 2;
	function clamp() {
		value = min;
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
