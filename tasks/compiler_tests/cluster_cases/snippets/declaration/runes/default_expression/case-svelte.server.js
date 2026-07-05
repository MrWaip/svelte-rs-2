import * as $ from "svelte/internal/server";
function s($$renderer, x = { a: 1 }) {
	$$renderer.push(`<button>${$.escape(x.a)}</button>`);
}
export default function App($$renderer) {
	let v = void 0;
	s($$renderer, v);
}
