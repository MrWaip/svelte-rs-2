import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let onclick = $.prop($$props, "onclick", 8, undefined);
	let useFn = $.prop($$props, "useFn", 8, undefined);
	let useArgs = $.prop($$props, "useArgs", 24, () => []);
	let href = $.prop($$props, "href", 8, undefined);
	function getTag() {
		return href() ? "a" : "div";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, getTag, false, ($$element, $$anchor) => {
		$.action($$element, ($$node, $$action_arg) => useFn()?.($$node, $$action_arg), () => useArgs() || []);
		$.effect(() => $.event("click", $$element, function(...$$args) {
			onclick()?.apply(this, $$args);
		}));
		$.attribute_effect($$element, () => ({ href: href() }));
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
}
