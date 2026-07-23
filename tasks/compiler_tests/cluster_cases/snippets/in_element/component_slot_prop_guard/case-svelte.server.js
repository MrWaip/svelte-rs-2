import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		function foo($$renderer) {
			$$renderer.push(`<b>hi</b>`);
		}
		Component($$renderer, {
			foo,
			$$slots: { foo: true }
		});
	}
}
