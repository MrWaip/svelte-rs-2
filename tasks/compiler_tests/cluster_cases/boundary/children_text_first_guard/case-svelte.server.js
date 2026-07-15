import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	function failed($$renderer) {
		$$renderer.push(`<p>f</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			$$renderer.push(`<!---->boundary ${$.escape(x)} text`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
