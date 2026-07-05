App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag_proxy($.proxy({ name: "a" }), "value");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const item = $.tag($.derived(() => value), "item");
			$.get(item);
			$.add_svelte_meta(() => Comp($$anchor, { get name() {
				return $.get(item).name;
			} }), "component", App, 8, 1, { componentTag: "Comp" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
