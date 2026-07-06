import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		a: 1,
		b: 2
	}, a = tmp.a, b = tmp.b;
	function bump() {
		a = a;
		b = b;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
