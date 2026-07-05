import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { children } = $$props;
	$$renderer.push(`<select>`);
	children($$renderer);
	$$renderer.push(`<!----><!></select>`);
}
