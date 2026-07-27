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
	var $$exports = { ...$.legacy_api() };
	{
		$.async($$anchor, void 0, [async () => (await $.track_reactivity_loss(delay($$props.x)))()], ($$anchor, $0) => {
			$.add_svelte_meta(() => Child($$anchor, { get value() {
				return $.get($0);
			} }), "component", App, 11, 0, { componentTag: "Child" });
		});
		$.next();
	}
	return $.pop($$exports);
}
