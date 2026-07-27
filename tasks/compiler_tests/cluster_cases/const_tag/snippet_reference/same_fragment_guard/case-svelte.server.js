import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	Comp($$renderer, {
		children: ($$renderer) => {
			const foo = "bar";
			$$renderer.push(`<!---->bar`);
		},
		$$slots: { default: true }
	});
}
