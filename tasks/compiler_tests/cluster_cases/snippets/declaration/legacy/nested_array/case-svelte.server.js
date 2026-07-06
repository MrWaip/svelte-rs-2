import * as $ from "svelte/internal/server";
function s($$renderer, [[a, b], [c, d]]) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
}
export default function App($$renderer) {
	let v = [[1, 2], [3, 4]];
	s($$renderer, v);
}
