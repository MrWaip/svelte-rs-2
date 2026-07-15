import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	function failed($$renderer, _, reset) {
		$$renderer.push(`<button>reset</button>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[!-->`);
		{
			$$renderer.push(`<!---->${$.escape(x)} loading`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
