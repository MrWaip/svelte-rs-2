import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 15);
	let Comp = $.proxy(A);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => Comp, ($$anchor, $$component) => {
		$$component($$anchor, {
			get value() {
				return value();
			},
			set value($$value) {
				value($$value);
			}
		});
	});
	$.append($$anchor, fragment);
	$.pop();
}
