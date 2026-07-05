import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { n } = $$props;
	function compute() {
		return n + 1;
	}
	$$renderer.push(`<!--[-->`);
	{
		const value = compute();
		function row($$renderer) {
			$$renderer.push(`<p>row</p>`);
		}
		$$renderer.push(`<div>`);
		row($$renderer);
		$$renderer.push(`<!----></div>`);
	}
	$$renderer.push(`<!--]-->`);
}
