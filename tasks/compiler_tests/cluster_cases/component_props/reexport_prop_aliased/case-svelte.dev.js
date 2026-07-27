App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let stuff = $.prop($$props, "stuff", 7);
	var $$exports = {
		...$.legacy_api(),
		get cool() {
			return stuff();
		},
		set cool($$value) {
			stuff($$value);
		}
	};
	return $.pop($$exports);
}
