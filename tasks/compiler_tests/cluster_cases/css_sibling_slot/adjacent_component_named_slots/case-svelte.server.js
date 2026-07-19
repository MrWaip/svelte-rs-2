import * as $ from "svelte/internal/server";
export default function App_1($$renderer) {
	let App;
	$$renderer.push(`<div class="a svelte-v8ftti"></div> `);
	App($$renderer, { $$slots: {
		a: ($$renderer) => {
			$$renderer.push(`<div class="b svelte-v8ftti" slot="a"></div>`);
		},
		b: ($$renderer) => {
			$$renderer.push(`<div class="c svelte-v8ftti" slot="b"><div class="d svelte-v8ftti"></div> <div class="e svelte-v8ftti"></div></div>`);
		}
	} });
	$$renderer.push(`<!----> <div class="f svelte-v8ftti"></div>`);
}
