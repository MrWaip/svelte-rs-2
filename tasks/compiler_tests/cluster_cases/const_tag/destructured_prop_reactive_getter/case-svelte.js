import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const simpleReactive = $.derived(() => $$props.data.foo);
			const computed_const = $.derived(() => {
				const { destr } = { destr: 1 };
				return { destr };
			});
			const simpleStatic = $.derived(() => 5);
			Child($$anchor, {
				get a() {
					return $.get(simpleReactive);
				},
				get b() {
					return $.get(computed_const).destr;
				},
				c: $.get(simpleStatic)
			});
		};
		$.if(node, ($$render) => {
			if ($$props.data) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
