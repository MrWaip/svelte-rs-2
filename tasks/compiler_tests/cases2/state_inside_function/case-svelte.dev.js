App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function createCounter() {
		let count = $.tag($.state(0), "count");
		return {
			get count() {
				return $.get(count);
			},
			increment() {
				$.update(count);
			}
		};
	}
	var $$exports = {
		...$.legacy_api(),
		get createCounter() {
			return createCounter;
		}
	};
	return $.pop($$exports);
}
