import * as $ from "svelte/internal/server";
function s($$renderer, [a, , c]) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(c)}</button>`);
}
export default function App($$renderer) {
	let v = [
		1,
		2,
		3
	];
	s($$renderer, v);
}
