import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	Wrap($$renderer, { $$slots: {
		image: ($$renderer) => {
			Inner($$renderer, { slot: "image" });
		},
		action: ($$renderer) => {
			$$renderer.push(`<span slot="action">0</span>`);
		}
	} });
}
