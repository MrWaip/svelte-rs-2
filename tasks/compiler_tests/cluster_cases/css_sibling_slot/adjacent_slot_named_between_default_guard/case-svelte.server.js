import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	Child($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<div class="a svelte-1uoiiwh">a</div> <div class="c svelte-1uoiiwh">c</div>`);
		},
		$$slots: {
			default: true,
			wut: ($$renderer) => {
				$$renderer.push(`<div class="b" slot="wut">b</div>`);
			}
		}
	});
}
