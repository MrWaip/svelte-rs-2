import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {}, a = $.fallback(tmp.a, 10), b = $.fallback(tmp.b, 20);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
