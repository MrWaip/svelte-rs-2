import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let refs = $.proxy([]);
	const Derived_1 = $.derived(() => Widget);
	var fragment = root();
	var node = $.first_child(fragment);
	$.component(node, () => $.get(Derived_1), ($$anchor, Derived_1_1) => {
		$.bind_this(Derived_1_1($$anchor, {}), ($$value) => refs[1] = $$value, () => refs?.[1]);
	});
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			const Const_0 = $.derived(() => Widget);
			var fragment_1 = $.comment();
			var node_2 = $.first_child(fragment_1);
			$.component(node_2, () => $.get(Const_0), ($$anchor, Const_0_1) => {
				$.bind_this(Const_0_1($$anchor, {}), ($$value) => refs[0] = $$value, () => refs?.[0]);
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node_1, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
