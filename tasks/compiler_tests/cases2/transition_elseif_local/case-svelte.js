import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root_1 = $.from_html(`<p>first</p>`);
var root_2 = $.from_html(`<div>second</div>`);
export default function App($$anchor) {
	let x = false;
	let y = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		var consequent_1 = ($$anchor) => {
			var div = root_2();
			$.transition(3, div, () => fade);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (x) $$render(consequent);
			else if (y) $$render(consequent_1, 1);
		});
	}
	$.append($$anchor, fragment);
}
