import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8, null);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const value = $.derived(() => {
				let [a] = $$slotProps.value;
				return { a };
			});
			const x = $.tag($.derived_safe_equal(() => ($.deep_read_state($.get(value).a), $.untrack(() => $.get(value).a ? $.get(value).a({ k: 1 }) : null))), "x");
			$.get(x);
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, $.get(x)));
			$.append($$anchor, text);
		} }
	}), "component", App, 6, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
