import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	Inner($$renderer, { $$slots: {
		a: ($$renderer) => {
			$$renderer.push(`<div slot="a"></div>`);
		},
		b: ($$renderer) => {
			$$renderer.push(`<div slot="b"></div>`);
		},
		c: ($$renderer) => {
			$$renderer.push(`<div slot="c">x</div>`);
		}
	} });
}
