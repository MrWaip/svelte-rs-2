App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	function getTotal() {
		const total = $.tag($.derived(() => {
			let sum = 0;
			for (const item of items) {
				sum += item;
			}
			return sum;
		}), "total");
		return $.get(total);
	}
	var $$exports = {
		...$.legacy_api(),
		get getTotal() {
			return getTotal;
		}
	};
	return $.pop($$exports);
}
