import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<span>child</span>`);
export default function App($$anchor) {
	let current = A;
	let cond = false;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, {
			children: ($$anchor, $$slotProps) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				{
					var consequent = ($$anchor) => {
						var span = root();
						$.append($$anchor, span);
					};
					$.if(node_1, ($$render) => {
						if (cond) $$render(consequent);
					});
				}
				$.append($$anchor, fragment_1);
			},
			$$slots: { default: true }
		});
	});
	$.append($$anchor, fragment);
}
