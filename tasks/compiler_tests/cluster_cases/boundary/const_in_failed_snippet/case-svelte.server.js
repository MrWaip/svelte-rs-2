import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { n } = $$props;
	function compute() {
		return n + 1;
	}
	function failed($$renderer) {
		$$renderer.push(`<p>failed</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			const value = compute();
			$$renderer.push(`<div>${$.escape(value)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
