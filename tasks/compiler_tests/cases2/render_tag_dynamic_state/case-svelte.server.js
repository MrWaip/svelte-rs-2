import * as $ from "svelte/internal/server";
function greeting($$renderer, name) {
	$$renderer.push(`<p>Hello ${$.escape(name)}</p>`);
}
export default function App($$renderer) {
	let show = null;
	show($$renderer);
	$$renderer.push(`<!---->`);
}
