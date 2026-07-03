import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.from_html(`<div>hello</div>`);
export default function App($$anchor) {
	var data, params;
	var $$promises = $.run([async () => data = await fetch("/api"), () => params = $.proxy(data.params)]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.run_after_blockers([$$promises[1]], () => {
				$.transition(3, div, () => fade, () => params);
			});
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
