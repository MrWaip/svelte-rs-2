import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, b } = $$props;
	(a || b)($$renderer);
	$$renderer.push(`<!---->`);
}
