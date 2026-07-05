import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const k = "z";
	let tmp = { z: 1 }, v = tmp[k];
	$$renderer.push(`<button>${$.escape(v)}</button>`);
}
