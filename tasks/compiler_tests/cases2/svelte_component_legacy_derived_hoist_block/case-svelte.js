import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let flag = $.prop($$props, "flag", 8, false);
	let Comp = $.prop($$props, "Comp", 8);
	function onA() {}
	function onB() {}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived_safe_equal(() => flag() ? onA : undefined);
		let $1 = $.derived_safe_equal(() => flag() ? onB : undefined);
		$.component(node, Comp, ($$anchor, $$component) => {
			$$component($$anchor, {
				get onA() {
					return $.get($0);
				},
				get onB() {
					return $.get($1);
				}
			});
		});
	}
	$.append($$anchor, fragment);
}
