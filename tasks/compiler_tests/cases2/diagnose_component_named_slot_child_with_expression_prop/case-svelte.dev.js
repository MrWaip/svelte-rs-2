import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
import Outer from "./Outer.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		$.add_svelte_meta(() => Inner($$anchor, {
			slot: "footer",
			get x() {
				return value();
			}
		}), "component", App, 10, 1, { componentTag: "Inner" });
	} } }), "component", App, 9, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
