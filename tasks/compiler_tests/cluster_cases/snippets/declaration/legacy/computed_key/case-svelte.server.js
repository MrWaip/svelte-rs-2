import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const k = "z";
	let v = { z: 1 };
	function s($$renderer, { [k]: v }) {
		$$renderer.push(`<button>${$.escape(v)}</button>`);
	}
	s($$renderer, v);
}
