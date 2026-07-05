import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
import { tracker } from "./tracker";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.init();
	{
		let $0 = $.derived_safe_equal(() => ($.deep_read_state(tracker), $.untrack(() => tracker.click.upgrade())));
		$.add_svelte_meta(() => Child($$anchor, { get track() {
			return $.get($0);
		} }), "component", App, 6, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
