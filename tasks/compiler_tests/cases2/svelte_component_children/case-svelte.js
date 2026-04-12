import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root_1 = $.from_html(`<span>child</span>`);
export default function App($$anchor) {
	let current = A;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, {
			answer: 42,
			children: ($$anchor, $$slotProps) => {
				var span = root_1();
				$.append($$anchor, span);
			},
			$$slots: { default: true }
		});
	});
	$.append($$anchor, fragment);
}
