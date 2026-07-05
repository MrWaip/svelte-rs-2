import * as $ from "svelte/internal/server";
function s($$renderer, x) {
	$$renderer.push(`<button>${$.escape(x)}</button>`);
}
export default function App($$renderer) {
	let v = 1;
	s($$renderer, v);
}
