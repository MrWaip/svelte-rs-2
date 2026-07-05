import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cond, a, b } = $$props;
	(cond ? a : b)($$renderer);
	$$renderer.push(`<!---->`);
}
