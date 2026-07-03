import * as $ from "svelte/internal/client";
import A from "./A.svelte";
import B from "./B.svelte";
var root = $.from_html(`<div>c</div>`);
var root_1 = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let data = null;
	let x = null;
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		const inner = ($$anchor) => {
			$.next();
			var text = $.text();
			text.nodeValue = "";
			$.append($$anchor, text);
		};
		A(node, {
			inner,
			$$slots: { inner: true }
		});
	}
	var node_1 = $.sibling(node, 2);
	B(node_1, {
		children: ($$anchor, $$slotProps) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			{
				var consequent = ($$anchor) => {
					var div = root();
					$.append($$anchor, div);
				};
				$.if(node_2, ($$render) => {
					if (data) $$render(consequent);
				});
			}
			$.append($$anchor, fragment_2);
		},
		$$slots: { default: true }
	});
	$.append($$anchor, fragment);
}
