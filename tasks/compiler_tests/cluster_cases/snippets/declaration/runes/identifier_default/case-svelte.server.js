import * as $ from "svelte/internal/server";
function s($$renderer, x = true) {
	$$renderer.push(`<button>${$.escape(x)}</button>`);
}
export default function App($$renderer) {
	let v = false;
	s($$renderer, v);
}
