import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 8, null);
	Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const value = $.derived(() => {
				let [a] = $$slotProps.value;
				return { a };
			});
			const x = $.derived_safe_equal(() => ($.deep_read_state($.get(value).a), $.untrack(() => $.get(value).a ? $.get(value).a({ k: 1 }) : null)));
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, $.get(x)));
			$.append($$anchor, text);
		} }
	});
}
