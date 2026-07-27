import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.prop($$props, "count", 7, 0);
	var $$exports = {
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
		}
	};
	return $.pop($$exports);
}
