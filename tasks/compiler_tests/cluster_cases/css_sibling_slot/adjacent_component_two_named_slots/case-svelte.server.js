import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, { $$slots: {
		x: ($$renderer) => {
			$$renderer.push(`<div class="a" slot="x">a</div>`);
		},
		y: ($$renderer) => {
			$$renderer.push(`<div class="b svelte-1k5tp9w" slot="y">b</div>`);
		}
	} });
	$$renderer.push(`<!----> <div class="c svelte-1k5tp9w">c</div>`);
}
