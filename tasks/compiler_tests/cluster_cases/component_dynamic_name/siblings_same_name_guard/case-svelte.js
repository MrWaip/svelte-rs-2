import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	const B = $.derived(() => A);
	var fragment = root();
	var node = $.first_child(fragment);
	$.component(node, () => $.get(B), ($$anchor, B_1) => {
		B_1($$anchor, {
			children: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text("one");
				$.append($$anchor, text);
			},
			$$slots: { default: true }
		});
	});
	var node_1 = $.sibling(node, 2);
	$.component(node_1, () => $.get(B), ($$anchor, B_2) => {
		B_2($$anchor, {
			children: ($$anchor, $$slotProps) => {
				$.next();
				var text_1 = $.text("two");
				$.append($$anchor, text_1);
			},
			$$slots: { default: true }
		});
	});
	$.append($$anchor, fragment);
}
