import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	A($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<span slot="default">x</span>`);
		},
		$$slots: { default: true }
	});
}
