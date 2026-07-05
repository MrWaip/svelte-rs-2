import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => A($$anchor, {
		children: "foo",
		$$slots: { default: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text("bar");
			$.append($$anchor, text);
		} }
	}), "component", App, 5, 0, { componentTag: "A" });
	return $.pop($$exports);
}
