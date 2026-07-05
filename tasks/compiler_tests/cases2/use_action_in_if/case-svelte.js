import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.from_html(`<div>text</div>`);
export default function App($$anchor) {
	let show = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.action(div, ($$node) => tooltip?.($$node));
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
