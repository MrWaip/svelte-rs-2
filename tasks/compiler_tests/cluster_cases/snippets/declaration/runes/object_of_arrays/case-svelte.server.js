import * as $ from "svelte/internal/server";
function s($$renderer, { p: [a, b], q: [c, d] }) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
}
export default function App($$renderer) {
	let v = {
		p: [1, 2],
		q: [3, 4]
	};
	s($$renderer, v);
}
