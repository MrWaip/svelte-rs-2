import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const foo = $$props.data.foo;
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, foo));
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($$props.data) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
