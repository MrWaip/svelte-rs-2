import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let name = $.prop($$props, "name", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var text = $.text();
			$.template_effect(() => $.set_text(text, `hello ${name() ?? ""}`));
			$.append($$anchor, text);
		}),
		$$slots: { default: true }
	}), "component", App, 6, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
