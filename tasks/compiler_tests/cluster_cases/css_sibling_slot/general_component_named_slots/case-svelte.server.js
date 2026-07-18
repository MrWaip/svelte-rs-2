import * as $ from "svelte/internal/server";
export default function App_1($$renderer) {
	let App;
	$$renderer.push(`<div class="a svelte-1u1mcs6"></div> `);
	App($$renderer, { $$slots: {
		a: ($$renderer) => {
			$$renderer.push(`<div class="b svelte-1u1mcs6" slot="a"></div>`);
		},
		b: ($$renderer) => {
			$$renderer.push(`<div class="c" slot="b"><div class="d svelte-1u1mcs6"></div> <div class="e svelte-1u1mcs6"></div></div>`);
		},
		c: ($$renderer) => {
			$$renderer.push(`<div class="f svelte-1u1mcs6" slot="c"></div>`);
		}
	} });
	$$renderer.push(`<!----> <div class="g svelte-1u1mcs6"></div>`);
}
