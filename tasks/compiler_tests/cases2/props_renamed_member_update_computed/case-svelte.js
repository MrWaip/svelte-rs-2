import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let local = $.prop($$props, "value", 23, () => ({ stats: { count: 0 } }));
	let key = "count";
	function bump() {
		local().stats[key]++;
	}
	$.pop();
}
