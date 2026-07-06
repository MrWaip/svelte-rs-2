import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { row } = $$props;
	row($$renderer);
	$$renderer.push(`<!---->`);
}
