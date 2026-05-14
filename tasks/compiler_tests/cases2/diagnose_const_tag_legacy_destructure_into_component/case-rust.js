import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const item = getItem();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.derived_safe_equal(() => {
				const { title, status } = item;
				return {
					title,
					status
				};
			});
			Child($$anchor, {
				get status() {
					return $.get(computed_const).status;
				},
				children: ($$anchor, $$slotProps) => {
					$.next();
					var text = $.text();
					$.template_effect(() => $.set_text(text, $.get(computed_const).title));
					$.append($$anchor, text);
				},
				$$slots: { default: true }
			});
		};
		$.if(node, ($$render) => {
			if (item) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
