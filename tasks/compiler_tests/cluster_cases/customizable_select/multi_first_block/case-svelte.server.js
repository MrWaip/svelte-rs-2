import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { header, footer } = $$props;
	$$renderer.push(`<select>`);
	header($$renderer);
	$$renderer.push(`<!----><div>x</div>`);
	footer($$renderer);
	$$renderer.push(`<!----><!></select>`);
}
