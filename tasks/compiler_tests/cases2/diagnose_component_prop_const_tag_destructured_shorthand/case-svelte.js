import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $value = () => $.store_get(value, "$value", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const value = writable({
		a: 1,
		b: 2
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived(() => {
				const { a, b } = $value();
				return {
					a,
					b
				};
			});
			Comp($$anchor, {
				get x() {
					return $.get(computed_const).a;
				},
				get b() {
					return $.get(computed_const).b;
				}
			});
		};
		$.if(node, ($$render) => {
			if ($value()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
