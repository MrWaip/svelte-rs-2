import * as $ from "svelte/internal/server";
function greeting($$renderer) {
	$$renderer.push(`<p>Hello</p>`);
}
export default function App($$renderer) {
	greeting?.($$renderer);
}
