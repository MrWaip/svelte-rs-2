import * as $ from "svelte/internal/server";
function s($$renderer, [[a, b] = [8, 9], c]) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
}
export default function App($$renderer) {
	let v = [[1, 2], 3];
	s($$renderer, v);
}
