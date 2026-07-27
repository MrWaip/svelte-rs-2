import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1> </h1>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const greet = ($$anchor) => {
				var h1 = root();
				var text = $.child(h1, true);
				$.reset(h1);
				$.template_effect(() => $.set_text(text, $.get(number)), void 0, void 0, [promises[0]]);
				$.append($$anchor, h1);
			};
			let number;
			var promises = $.run([async () => number = (await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))()]);
			greet($$anchor);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
