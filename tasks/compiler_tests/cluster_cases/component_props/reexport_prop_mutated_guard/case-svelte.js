import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let stuff = $.prop($$props, "stuff", 7);
	function bump() {
		stuff(5);
	}
	var $$exports = {
		get stuff() {
			return stuff();
		},
		set stuff($$value) {
			stuff($$value);
		}
	};
	return $.pop($$exports);
}
