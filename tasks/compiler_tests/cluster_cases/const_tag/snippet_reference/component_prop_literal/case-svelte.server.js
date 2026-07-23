import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		function prop($$renderer) {
			$$renderer.push(`<!---->bar`);
		}
		Comp($$renderer, {
			prop,
			children: ($$renderer) => {
				const foo = "bar";
			},
			$$slots: {
				prop: true,
				default: true
			}
		});
	}
}
