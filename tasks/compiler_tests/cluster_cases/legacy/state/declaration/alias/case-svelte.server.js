import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		a: 1,
		b: 2
	}, x = tmp.a, y = tmp.b;
	function bump() {
		x = x;
		y = y;
	}
	$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
}
