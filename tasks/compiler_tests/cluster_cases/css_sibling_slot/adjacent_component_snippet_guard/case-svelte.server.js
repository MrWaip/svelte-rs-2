import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div><x class="svelte-zsofz2"></x> `);
	{
		function foo($$renderer) {
			$$renderer.push(`<v class="svelte-zsofz2"></v>`);
		}
		Child($$renderer, {
			foo,
			children: ($$renderer) => {
				$$renderer.push(`<y class="svelte-zsofz2"></y>`);
			},
			$$slots: {
				foo: true,
				default: true
			}
		});
	}
	$$renderer.push(`<!----> <z class="svelte-zsofz2"></z> `);
	{
		function foo($$renderer) {
			$$renderer.push(`<span><n></n></span>`);
		}
		Child($$renderer, {
			foo,
			children: ($$renderer) => {
				$$renderer.push(`<span><n></n></span>`);
			},
			$$slots: {
				foo: true,
				default: true
			}
		});
	}
	$$renderer.push(`<!----> <m></m></div>`);
}
