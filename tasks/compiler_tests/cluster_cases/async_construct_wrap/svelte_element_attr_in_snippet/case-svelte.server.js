import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	async function g() {
		return 2;
	}
	{
		function foo($$renderer) {
			$$renderer.child(async ($$renderer) => {
				const $$0 = (await $.save(g()))();
				$.element($$renderer, "div", () => {
					$$renderer.push(`${$.attr("title", $$0)}`);
				});
			});
		}
		Child($$renderer, {
			foo,
			$$slots: { foo: true }
		});
	}
}
