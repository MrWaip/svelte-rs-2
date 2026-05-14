import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		var alternate = ($$anchor) => {
			var text_1 = $.text("b");
			$.append($$anchor, text_1);
		};
		$.if(node, ($$render) => {
			if ($$props.foo.bar === "x") $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
