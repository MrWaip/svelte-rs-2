import * as $ from "svelte/internal/server";
function s($$renderer, { a, ...rest }) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.b)}</button>`);
}
export default function App($$renderer) {
	let v = {
		a: 1,
		b: 2,
		c: 3
	};
	s($$renderer, v);
}
