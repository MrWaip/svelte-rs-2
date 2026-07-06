import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function f(a) {
		return a;
	}
	let count = 0;
	const r = f(true);
	$$renderer.push(`<button>${$.escape(r)}${$.escape(count)}</button>`);
}
