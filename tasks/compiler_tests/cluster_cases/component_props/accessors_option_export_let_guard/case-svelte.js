import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let stuff = $.prop($$props, "stuff", 12);
	let count = $.prop($$props, "count", 12, 0);
	var $$exports = {
		get stuff() {
			return stuff();
		},
		set stuff($$value) {
			stuff($$value);
			$.flush();
		},
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
			$.flush();
		}
	};
	return $.pop($$exports);
}
