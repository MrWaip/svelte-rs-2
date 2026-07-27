import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	async function f() {
		return 1;
	}
	async function g() {
		return 2;
	}
	{
		$.async($$anchor, void 0, [() => f()], ($$anchor, $0) => {
			Child($$anchor, {
				get a() {
					return $.get($0);
				},
				children: ($$anchor, $$slotProps) => {
					var fragment_1 = $.comment();
					var node = $.first_child(fragment_1);
					$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
						$.attribute_effect($$element, ($0) => ({ title: $0 }), void 0, [() => g()]);
					});
					$.append($$anchor, fragment_1);
				},
				$$slots: { default: true }
			});
		});
		$.next();
	}
}
