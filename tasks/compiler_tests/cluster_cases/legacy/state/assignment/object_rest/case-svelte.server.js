import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 0;
	let rest = 0;
	let obj = {
		a: 1,
		x: 2
	};
	function update() {
		({a, ...rest} = obj);
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest)}</button>`);
}
