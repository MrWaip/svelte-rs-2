App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Button($$anchor, {
		theme: "primary",
		onclick: () => $.update(count),
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, `Clicked ${$.get(count) ?? ""} times`));
			$.append($$anchor, text);
		}),
		$$slots: { default: true }
	}), "component", App, 6, 0, { componentTag: "Button" });
	return $.pop($$exports);
}
