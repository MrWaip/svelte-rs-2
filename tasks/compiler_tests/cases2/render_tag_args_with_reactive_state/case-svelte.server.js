import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = 0;
	let { row } = $$props;
	row($$renderer, count + 1);
	$$renderer.push(`<!---->`);
}
