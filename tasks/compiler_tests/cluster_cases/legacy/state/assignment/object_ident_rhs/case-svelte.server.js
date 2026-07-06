import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let b = 0;
	let obj = {
		a: 1,
		b: 2
	};
	function update() {
		({a, b} = obj);
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
