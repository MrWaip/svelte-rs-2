App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const store = { sel: { y: 1 } };
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const a = $.tag($.derived(() => store.sel), "a");
			$.get(a);
			$.add_svelte_meta(() => Comp($$anchor, { get foo() {
				return $.get(a).y;
			} }), "component", App, 9, 1, { componentTag: "Comp" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
