import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function f(a) {
		return a + 1;
	}
	let count = 0;
	const r = f(1);
	$$renderer.push(`<button>${$.escape(r)}${$.escape(count)}</button>`);
}
