import * as $ from "svelte/internal/client";
import { Kind } from "./kinds";
var root_1 = $.from_html(`<span>A</span>`);
var root_2 = $.from_html(`<span>B</span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		var alternate = ($$anchor) => {
			var span_1 = root_2();
			$.append($$anchor, span_1);
		};
		$.if(node, ($$render) => {
			if ($$props.item.kind === Kind.A) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
