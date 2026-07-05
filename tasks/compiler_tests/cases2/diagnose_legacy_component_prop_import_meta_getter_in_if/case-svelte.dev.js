import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let cond = $.prop($$props, "cond", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.add_svelte_meta(() => Inner($$anchor, { get url() {
				return $.untrack(() => import.meta.env.VITE_X);
			} }), "component", App, 8, 1, { componentTag: "Inner" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (cond()) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
