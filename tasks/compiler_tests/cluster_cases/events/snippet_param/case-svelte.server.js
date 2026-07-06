import * as $ from "svelte/internal/server";
function row($$renderer, handler) {
	$$renderer.push(`<button>x</button>`);
}
export default function App($$renderer) {
	row($$renderer, () => {});
}
