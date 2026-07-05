import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	let visible = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.transition(3, div, () => fade);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
