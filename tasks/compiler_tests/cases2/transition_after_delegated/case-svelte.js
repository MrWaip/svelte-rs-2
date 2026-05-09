import * as $ from "svelte/internal/client";
import { slide } from "svelte/transition";
var root_1 = $.from_html(`<div>hi</div>`);
export default function App($$anchor) {
	let visible = true;
	function k() {}
	function c() {}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root_1();
			$.delegated("keydown", div, k);
			$.delegated("click", div, c);
			$.transition(3, div, () => slide);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
$.delegate(["keydown", "click"]);
