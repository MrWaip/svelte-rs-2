import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { opt } = $$props;
	$$renderer.push(`<select>`);
	opt($$renderer);
	$$renderer.push(`<!----><!></select> <select>`);
	opt($$renderer);
	$$renderer.push(`<!----><!></select>`);
}
