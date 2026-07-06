import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	const double = $.derived(() => count * 2);
	$$renderer.push(`<button>${$.escape(double())}</button>`);
}
