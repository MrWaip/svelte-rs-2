import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button>+</button>`);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var button = root_1();
			$.template_effect(() => {
				console.log({ count: $.snapshot($.get(count)) });
				debugger;
			});
			$.delegated("click", button, () => $.update(count));
			$.append($$anchor, button);
		};
		$.if(node, ($$render) => {
			if ($.get(count)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
