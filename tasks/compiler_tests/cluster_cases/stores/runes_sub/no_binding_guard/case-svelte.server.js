import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let foo = 0;
	let bar = $.derived(() => foo + 1);
	$$renderer.push(`<button>${$.escape(foo)} ${$.escape(bar())}</button>`);
}
