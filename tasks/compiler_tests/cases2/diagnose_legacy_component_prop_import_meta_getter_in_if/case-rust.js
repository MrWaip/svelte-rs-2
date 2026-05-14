import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let cond = $.prop($$props, "cond", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			Inner($$anchor, { get url() {
				return $.untrack(() => import.meta.env.VITE_X);
			} });
		};
		$.if(node, ($$render) => {
			if (cond()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
