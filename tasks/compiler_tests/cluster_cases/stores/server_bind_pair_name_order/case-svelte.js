import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var bind_get = () => $$props.aGet();
			var bind_set = $$props.aSet;
			Child($$anchor, {
				get value() {
					return bind_get();
				},
				set value($$value) {
					bind_set($$value);
				}
			});
		};
		var alternate = ($$anchor) => {
			var bind_get_1 = () => $$props.bGet();
			var bind_set_1 = $$props.bSet;
			Child($$anchor, {
				get value() {
					return bind_get_1();
				},
				set value($$value) {
					bind_set_1($$value);
				}
			});
		};
		$.if(node, ($$render) => {
			if ($$props.cond) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
