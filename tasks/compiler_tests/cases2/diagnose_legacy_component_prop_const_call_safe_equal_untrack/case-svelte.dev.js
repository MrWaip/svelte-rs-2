import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8, 0);
	const tracker = { click: () => 1 };
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived_safe_equal(() => $.untrack(() => tracker.click()));
		$.add_svelte_meta(() => Child($$anchor, {
			get left() {
				return $.get($0);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text();
				$.template_effect(() => $.set_text(text, x()));
				$.append($$anchor, text);
			}),
			$$slots: { default: true }
		}), "component", App, 7, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
