App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	function getDoubled() {
		const doubled = $.tag($.derived(() => count * 2), "doubled");
		return $.get(doubled);
	}
	var $$exports = {
		...$.legacy_api(),
		get getDoubled() {
			return getDoubled;
		}
	};
	return $.pop($$exports);
}
