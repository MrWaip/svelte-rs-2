App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 0;
	function getInfo() {
		const computed = $.tag($.derived(() => value * 2), "computed");
		return { computed: $.get(computed) };
	}
	var $$exports = {
		...$.legacy_api(),
		get getInfo() {
			return getInfo;
		}
	};
	return $.pop($$exports);
}
