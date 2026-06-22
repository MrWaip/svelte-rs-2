import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const Component = $.derived(() => $$props.component);
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.component(node_1, () => $.get(Component), ($$anchor, Component_1) => {
				Component_1($$anchor, {});
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if ($$props.component) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
