import * as $ from "svelte/internal/server";
function s($$renderer, { "a-b": ab, "c d": cd }) {
	$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
}
export default function App($$renderer) {
	let v = {
		"a-b": 1,
		"c d": 2
	};
	s($$renderer, v);
}
