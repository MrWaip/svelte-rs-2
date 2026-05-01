App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Button($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("hello");
			$.append($$anchor, text);
		}),
		$$slots: { default: true }
	}), "component", App, 5, 0, { componentTag: "Button" });
	return $.pop($$exports);
}
