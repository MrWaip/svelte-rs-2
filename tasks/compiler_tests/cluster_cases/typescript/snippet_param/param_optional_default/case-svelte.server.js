import * as $ from "svelte/internal/server";
function row($$renderer, c = 5) {
	$$renderer.push(`<span>${$.escape(c)}</span>`);
}
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}</button> `);
	row($$renderer, count);
	$$renderer.push(`<!---->`);
}
