import * as $ from "svelte/internal/server";
function s($$renderer, { p: { a }, q: { b } }) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
}
export default function App($$renderer) {
	let v = {
		p: { a: 1 },
		q: { b: 2 }
	};
	s($$renderer, v);
}
