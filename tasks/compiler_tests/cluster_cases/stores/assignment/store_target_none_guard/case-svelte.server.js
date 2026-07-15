import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a;
	let b;
	const obj = {
		a: 1,
		b: 2
	};
	function run() {
		({a, b} = obj);
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
