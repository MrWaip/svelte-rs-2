import * as $ from "svelte/internal/server";
function greeting($$renderer, msg) {
	$$renderer.push(`<p>Hello ${$.escape(msg)}</p>`);
}
export default function App($$renderer) {
	let name = "world";
	$$renderer.push(`<p>world</p>`);
}
