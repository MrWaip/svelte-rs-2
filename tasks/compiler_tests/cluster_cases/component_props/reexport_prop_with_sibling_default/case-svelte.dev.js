App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.prop($$props, "count", 7, 0), stuff = $.prop($$props, "stuff", 7);
	var $$exports = {
		...$.legacy_api(),
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
		},
		get stuff() {
			return stuff();
		},
		set stuff($$value) {
			stuff($$value);
		}
	};
	return $.pop($$exports);
}
