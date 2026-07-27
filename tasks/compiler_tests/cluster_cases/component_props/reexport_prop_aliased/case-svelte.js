import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let stuff = $.prop($$props, "stuff", 7);
	var $$exports = {
		get cool() {
			return stuff();
		},
		set cool($$value) {
			stuff($$value);
		}
	};
	return $.pop($$exports);
}
