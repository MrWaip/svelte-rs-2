import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root_1 = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	let show = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root_1();
			$.attach(div, () => tooltip);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
