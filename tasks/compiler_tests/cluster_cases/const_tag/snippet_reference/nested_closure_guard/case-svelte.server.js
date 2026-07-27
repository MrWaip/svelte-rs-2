import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	Comp($$renderer, {
		children: ($$renderer) => {
			const foo = "bar";
			{
				function prop($$renderer) {
					$$renderer.push(`<!---->bar`);
				}
				Inner($$renderer, {
					prop,
					$$slots: { prop: true }
				});
			}
		},
		$$slots: { default: true }
	});
}
