import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { markup } = $$props;
	$$renderer.push(`<select>${$.html(markup)}<!></select>`);
}
