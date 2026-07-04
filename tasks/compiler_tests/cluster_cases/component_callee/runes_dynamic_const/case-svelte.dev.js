App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const Component = $.tag($.derived(() => $$props.component), "Component");
			$.get(Component);
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.add_svelte_meta(() => $.component(node_1, () => $.get(Component), ($$anchor, Component_1) => {
				Component_1($$anchor, {});
			}), "component", App, 6, 1, { componentTag: "Component" });
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.component) $$render(consequent);
		}), "if", App, 4, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
