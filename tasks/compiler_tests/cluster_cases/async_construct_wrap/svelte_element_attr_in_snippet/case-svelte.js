import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	async function g() {
		return 2;
	}
	{
		const foo = ($$anchor) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			$.element(node, () => "div", false, ($$element, $$anchor) => {
				$.attribute_effect($$element, ($0) => ({ title: $0 }), void 0, [() => g()]);
			});
			$.append($$anchor, fragment_1);
		};
		Child($$anchor, {
			foo,
			$$slots: { foo: true }
		});
	}
}
