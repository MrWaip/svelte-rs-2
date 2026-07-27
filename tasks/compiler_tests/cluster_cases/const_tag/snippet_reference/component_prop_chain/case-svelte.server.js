import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		function prop($$renderer) {
			$$renderer.push(`<!---->2`);
		}
		Comp($$renderer, {
			prop,
			children: ($$renderer) => {
				const a = 1;
				const foo = a + 1;
			},
			$$slots: {
				prop: true,
				default: true
			}
		});
	}
}
