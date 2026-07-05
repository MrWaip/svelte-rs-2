import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	let current = A;
	let items = [
		1,
		2,
		3
	];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, {
			children: ($$anchor, $$slotProps) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				$.each(node_1, 1, () => items, $.index, ($$anchor, item) => {
					var span = root();
					var text = $.child(span, true);
					$.reset(span);
					$.template_effect(() => $.set_text(text, $.get(item)));
					$.append($$anchor, span);
				});
				$.append($$anchor, fragment_1);
			},
			$$slots: { default: true }
		});
	});
	$.append($$anchor, fragment);
}
