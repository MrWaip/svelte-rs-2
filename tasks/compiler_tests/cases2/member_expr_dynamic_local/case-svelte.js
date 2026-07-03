import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let obj = null;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, obj.name));
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (obj) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
