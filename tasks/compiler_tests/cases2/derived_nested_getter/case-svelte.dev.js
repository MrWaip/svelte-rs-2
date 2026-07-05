App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 0;
	function makeAccessor() {
		const computed = $.tag($.derived(() => value + 1), "computed");
		return { get computed() {
			return $.get(computed);
		} };
	}
	var $$exports = {
		...$.legacy_api(),
		get makeAccessor() {
			return makeAccessor;
		}
	};
	return $.pop($$exports);
}
