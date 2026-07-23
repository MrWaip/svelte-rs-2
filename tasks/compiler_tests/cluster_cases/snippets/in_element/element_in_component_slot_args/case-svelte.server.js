import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	Component($$renderer, {
		children: ($$renderer) => {
			function children($$renderer, { with_prop }) {
				$$renderer.push(`<!---->txt ${$.escape(with_prop)}`);
			}
			$$renderer.push(`<span></span>`);
		},
		$$slots: { default: true }
	});
}
