App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = 0;
	function getValues() {
		const doubled = $.tag($.derived(() => x * 2), "doubled");
		return {
			doubled: $.get(doubled),
			get live() {
				return $.get(doubled);
			}
		};
	}
	var $$exports = {
		...$.legacy_api(),
		get getValues() {
			return getValues;
		}
	};
	return $.pop($$exports);
}
