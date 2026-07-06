import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let items = [
		1,
		2,
		3
	];
	function failed($$renderer, error) {
		$$renderer.push(`<p>${$.escape(x)}: ${$.escape(error.message)}</p>`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			const x = items.length;
			$$renderer.push(`<p>${$.escape(x)}</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
