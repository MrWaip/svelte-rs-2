import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	let n = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let a;
			let b;
			var promises = $.run([async () => a = await $.async_derived(() => Promise.resolve(n)), () => b = $.derived(() => $.get(a) * 2)]);
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, $.get(b)), void 0, void 0, [promises[1]]);
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if (n) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
