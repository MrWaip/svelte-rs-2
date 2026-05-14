import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<!> <div class="cap"><!></div>`, 1);
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	var div = root();
	var node = $.child(div);
	{
		const footer = ($$anchor) => {
			var fragment = root_1();
			var node_1 = $.first_child(fragment);
			$.component(node_1, () => $$props.Btn, ($$anchor, Btn_1) => {
				Btn_1($$anchor, {});
			});
			var div_1 = $.sibling(node_1, 2);
			var node_2 = $.child(div_1);
			$.component(node_2, () => $$props.Cap, ($$anchor, Cap_1) => {
				Cap_1($$anchor, {});
			});
			$.reset(div_1);
			$.append($$anchor, fragment);
		};
		$.component(node, () => $$props.Layout, ($$anchor, Layout_1) => {
			Layout_1($$anchor, {
				footer,
				$$slots: { footer: true }
			});
		});
	}
	$.reset(div);
	$.append($$anchor, div);
}
