import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let href = $.prop($$props, "href", 8, undefined);
	function getTag() {
		return href() ? "a" : "div";
	}
	function getRole() {
		return href() ? "link" : undefined;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, getTag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, ($0) => ({
			role: $0,
			href: href()
		}), [() => $.untrack(getRole)]);
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
