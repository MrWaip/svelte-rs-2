import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	Component($$renderer, {
		children: ($$renderer) => {
			function foo($$renderer) {
				$$renderer.push(`<b>hi</b>`);
			}
			$$renderer.push(`<div></div>`);
		},
		$$slots: { default: true }
	});
}
