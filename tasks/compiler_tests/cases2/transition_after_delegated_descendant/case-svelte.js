import * as $ from "svelte/internal/client";
import { slide } from "svelte/transition";
var root = $.from_html(`<div><button>hi</button></div>`);
export default function App($$anchor) {
	let visible = true;
	function k() {}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			var button = $.child(div);
			$.reset(div);
			$.delegated("click", button, k);
			$.transition(3, div, () => slide);
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (visible) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
