import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	C($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<!---->OMG`);
		},
		$$slots: {
			default: true,
			s: ($$renderer) => {
				$$renderer.push(`<x slot="s">1</x>`);
			}
		}
	});
}
