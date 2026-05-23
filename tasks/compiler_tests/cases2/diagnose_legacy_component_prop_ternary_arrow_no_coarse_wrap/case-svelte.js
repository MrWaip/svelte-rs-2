import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let isButton = $.prop($$props, "isButton", 8);
	let onAction = $.prop($$props, "onAction", 8);
	let item = $.prop($$props, "item", 8);
	$.init();
	{
		let $0 = $.derived_safe_equal(() => isButton() ? () => onAction()(item()) : undefined);
		Child($$anchor, { get onclick() {
			return $.get($0);
		} });
	}
	$.pop();
}
