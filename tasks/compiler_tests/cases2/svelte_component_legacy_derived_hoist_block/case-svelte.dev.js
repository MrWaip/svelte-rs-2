import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let flag = $.prop($$props, "flag", 8, false);
	let Comp = $.prop($$props, "Comp", 8);
	function onA() {}
	function onB() {}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived_safe_equal(() => flag() ? onA : undefined);
		let $1 = $.derived_safe_equal(() => flag() ? onB : undefined);
		$.add_svelte_meta(() => $.component(node, Comp, ($$anchor, $$component) => {
			$$component($$anchor, {
				get onA() {
					return $.get($0);
				},
				get onB() {
					return $.get($1);
				}
			});
		}), "component", App, 10, 0, { componentTag: "svelte:component" });
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
