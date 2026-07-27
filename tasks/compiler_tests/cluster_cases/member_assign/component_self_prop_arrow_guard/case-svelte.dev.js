App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ x: null }), "obj");
	let src = $.tag_proxy($.proxy({}), "src");
	let depth = 0;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.add_svelte_meta(() => App(node_1, {
				onChange: (v) => $.assign(obj, "x", "=", src, "(unknown):8:32"),
				depth: depth - 1
			}), "component", App, 8, 1, { componentTag: "svelte:self" });
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (depth) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
