import * as $ from "svelte/internal/server";
function s($$renderer, [a = 10, b = 20]) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
export default function App($$renderer) {
	let v = [1];
	s($$renderer, v);
}
