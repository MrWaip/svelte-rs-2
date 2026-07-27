import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		function prop($$renderer) {
			const foo = "bar";
			$$renderer.push(`<!---->bar`);
		}
		Comp($$renderer, {
			prop,
			$$slots: { prop: true }
		});
	}
}
