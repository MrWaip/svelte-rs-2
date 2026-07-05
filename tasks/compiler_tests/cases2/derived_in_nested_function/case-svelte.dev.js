App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = 1;
	let b = 2;
	function compute() {
		const sum = $.tag($.derived(() => a + b), "sum");
		return $.get(sum);
	}
	var $$exports = {
		...$.legacy_api(),
		get compute() {
			return compute;
		}
	};
	return $.pop($$exports);
}
