import * as $ from "svelte/internal/server";
function s($$renderer, { p: { a } = {} }) {
	$$renderer.push(`<button>${$.escape(a)}</button>`);
}
export default function App($$renderer) {
	let v = {};
	s($$renderer, v);
}
