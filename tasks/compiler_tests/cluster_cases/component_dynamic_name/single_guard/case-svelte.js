import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor) {
	const B = $.derived(() => A);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => $.get(B), ($$anchor, B_1) => {
		B_1($$anchor, {
			children: ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text("test");
				$.append($$anchor, text);
			},
			$$slots: { default: true }
		});
	});
	$.append($$anchor, fragment);
}
