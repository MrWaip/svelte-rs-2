import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let { items } = $$props;
	function foo($$renderer, a) {
		$$renderer.push(`<span>${$.escape(items)} ${$.escape(a)}</span>`);
	}
	Child($$renderer, {
		children: ($$renderer) => {
			foo($$renderer, 1);
		},
		$$slots: { default: true }
	});
}
