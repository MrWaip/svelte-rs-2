import * as $ from "svelte/internal/server";
function s($$renderer, { a: x, b: y }) {
	$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
}
export default function App($$renderer) {
	let v = {
		a: 1,
		b: 2
	};
	s($$renderer, v);
}
