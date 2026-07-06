import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { extra } = $$props;
	$$renderer.push(`<select><div>x</div>`);
	extra($$renderer);
	$$renderer.push(`<!----><!></select>`);
}
