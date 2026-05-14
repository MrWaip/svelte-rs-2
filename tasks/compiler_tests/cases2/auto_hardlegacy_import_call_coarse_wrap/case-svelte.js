import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
import { tracker } from "./tracker";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	{
		let $0 = $.derived_safe_equal(() => ($.deep_read_state(tracker), $.untrack(() => tracker.click.upgrade())));
		Child($$anchor, { get track() {
			return $.get($0);
		} });
	}
	$.pop();
}
