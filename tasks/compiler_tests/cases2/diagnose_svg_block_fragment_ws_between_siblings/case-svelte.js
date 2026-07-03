import * as $ from "svelte/internal/client";
var root = $.from_svg(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`, 1);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			$.next();
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (cond) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
