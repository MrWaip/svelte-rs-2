import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 1;
	let b = 2;
	function bump() {
		a += 1;
		b += 1;
	}
	$$renderer.push(`<button>${$.escape(a)}-${$.escape(b)}</button>`);
}
