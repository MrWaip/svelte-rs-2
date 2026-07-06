import * as $ from "svelte/internal/server";
function s($$renderer, { a, b }) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
export default function App($$renderer) {
	let v = {
		a: 1,
		b: 2
	};
	s($$renderer, v);
}
