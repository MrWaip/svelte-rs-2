import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function failed($$renderer, error) {
		$$renderer.push(`<p>${$.escape(error.message)}</p> `);
		helper($$renderer);
		$$renderer.push(`<!---->`);
	}
	$$renderer.boundary({ failed }, ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		{
			function helper($$renderer) {
				$$renderer.push(`<span>helper text</span>`);
			}
			$$renderer.push(`<p>content</p>`);
		}
		$$renderer.push(`<!--]-->`);
	});
}
