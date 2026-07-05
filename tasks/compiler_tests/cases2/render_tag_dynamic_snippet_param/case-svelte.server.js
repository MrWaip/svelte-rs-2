import * as $ from "svelte/internal/server";
function wrapper($$renderer, inner) {
	$$renderer.push(`<div>`);
	inner($$renderer);
	$$renderer.push(`<!----></div>`);
}
function greeting($$renderer) {
	$$renderer.push(`<p>Hello</p>`);
}
export default function App($$renderer) {
	let msg = "hi";
	wrapper($$renderer, greeting);
}
