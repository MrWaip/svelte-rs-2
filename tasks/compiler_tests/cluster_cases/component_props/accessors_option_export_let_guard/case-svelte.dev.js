import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let stuff = $.prop($$props, "stuff", 12);
	let count = $.prop($$props, "count", 12, 0);
	var $$exports = {
		...$.legacy_api(),
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
