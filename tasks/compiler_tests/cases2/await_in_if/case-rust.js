import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	let show = true;
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.await(node_1, () => promise, null, ($$anchor, value) => {
				var p = root_2();
				var text = $.child(p, true);
				$.reset(p);
				$.template_effect(() => $.set_text(text, $.get(value)));
				$.append($$anchor, p);
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (show) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
