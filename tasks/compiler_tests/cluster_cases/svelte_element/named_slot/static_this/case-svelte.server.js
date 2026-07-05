import * as $ from "svelte/internal/server";
import Foo from "./Foo.svelte";
export default function App($$renderer) {
	Foo($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<span>default</span>`);
		},
		$$slots: {
			default: true,
			other: ($$renderer) => {
				$.element($$renderer, "span", () => {
					$$renderer.push(` slot="other"`);
				}, () => {
					$$renderer.push(`This is other slot`);
				});
			}
		}
	});
}
