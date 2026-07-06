import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { w: 0 };
	$$renderer.push(`<div>${$.escape(obj.w)}</div>`);
}
