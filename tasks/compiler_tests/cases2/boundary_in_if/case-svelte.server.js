import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = true;
	if (show) {
		$$renderer.push("<!--[0-->");
		function failed($$renderer, error) {
			$$renderer.push(`<p>${$.escape(error.message)}</p>`);
		}
		$$renderer.boundary({ failed }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			{
				$$renderer.push(`<p>guarded</p>`);
			}
			$$renderer.push(`<!--]-->`);
		});
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
