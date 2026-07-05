import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { n } = $$props;
	function failed($$renderer) {
		$$renderer.push(`<p>failed</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<div>${$.escape(n)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
