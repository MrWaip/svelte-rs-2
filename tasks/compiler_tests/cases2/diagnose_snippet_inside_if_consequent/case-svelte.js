import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	{
		const body = ($$anchor) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			{
				var consequent = ($$anchor) => {
					var div = root();
					{
						const inner = ($$anchor) => {
							$.next();
							var text = $.text();
							$.template_effect(() => $.set_text(text, $$props.data?.flag?.text));
							$.append($$anchor, text);
						};
					}
					$.append($$anchor, div);
				};
				$.if(node, ($$render) => {
					if ($$props.data?.flag) $$render(consequent);
				});
			}
			$.append($$anchor, fragment_1);
		};
		Outer($$anchor, {
			body,
			$$slots: { body: true }
		});
	}
	$.pop();
}
