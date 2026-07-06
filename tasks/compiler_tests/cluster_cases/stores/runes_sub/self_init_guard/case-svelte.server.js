import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let state = 0;
	let derived = $.derived(() => state + 1);
	$$renderer.push(`<button>${$.escape(state)} ${$.escape(derived())}</button>`);
}
