import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = null;
	let b = null;
	let c = null;
	function load(source) {
		[a, b, c] = source();
	}
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
