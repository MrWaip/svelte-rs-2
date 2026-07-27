import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var b;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => b = true]);
	var $$exports = { ...$.legacy_api() };
	{
		$.async($$anchor, [$$promises[1]], void 0, ($$anchor) => {
			$.add_svelte_meta(() => Child($$anchor, { get b() {
				return b;
			} }), "component", App, 8, 0, { componentTag: "Child" });
		});
		$.next();
	}
	return $.pop($$exports);
}
