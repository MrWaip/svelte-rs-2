import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 0;
	function clamp(x) {
		value = Math.min(100, +x);
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
