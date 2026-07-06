import * as $ from "svelte/internal/server";
function s($$renderer, [a, ...rest]) {
	$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
}
export default function App($$renderer) {
	let v = [
		1,
		2,
		3
	];
	s($$renderer, v);
}
