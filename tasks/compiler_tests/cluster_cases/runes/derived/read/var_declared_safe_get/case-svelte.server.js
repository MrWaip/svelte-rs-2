import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	console.log(double?.());
	let count = 0;
	var double = $.derived(() => count * 2);
	$$renderer.push(`<button>${$.escape(double?.())}</button>`);
}
