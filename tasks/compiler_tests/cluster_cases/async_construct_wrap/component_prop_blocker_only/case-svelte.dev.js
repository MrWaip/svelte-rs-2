import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function delay(value) {
		return Promise.resolve(value);
	}
	var loaded;
	var $$promises = $.run([async () => loaded = (await $.track_reactivity_loss(delay(1)))()]);
	var $$exports = { ...$.legacy_api() };
	{
		$.async($$anchor, [$$promises[0]], void 0, ($$anchor) => {
			$.add_svelte_meta(() => Child($$anchor, { get value() {
				return loaded;
			} }), "component", App, 11, 0, { componentTag: "Child" });
		});
		$.next();
	}
	return $.pop($$exports);
}
