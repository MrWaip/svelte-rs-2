import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { children } = $$props;
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}</button> `);
	children($$renderer, count);
	$$renderer.push(`<!---->`);
}
