import * as $ from "svelte/internal/server";
function greeting($$renderer, msg) {
	$$renderer.push(`<p>Hello ${$.escape(msg)}</p>`);
}
export default function App($$renderer) {
	greeting($$renderer, "world");
}
