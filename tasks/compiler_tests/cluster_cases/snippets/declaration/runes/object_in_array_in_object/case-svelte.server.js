import * as $ from "svelte/internal/server";
function s($$renderer, { outer: [{ inner }] }) {
	$$renderer.push(`<button>${$.escape(inner)}</button>`);
}
export default function App($$renderer) {
	let v = { outer: [{ inner: 1 }] };
	s($$renderer, v);
}
