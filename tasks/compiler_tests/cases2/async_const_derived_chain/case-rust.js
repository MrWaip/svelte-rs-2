import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var data;
	var $$promises = $.run([async () => data = await fetch("/api")]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let a;
			let b;
			let c;
			var promises = $.run([
				() => $$promises[0].promise,
				() => a = $.derived(() => data.value),
				() => b = $.derived(() => $.get(a) * 2),
				() => c = $.derived(() => $.get(b) + 1)
			]);
			var p = root_1();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(c)), void 0, void 0, [promises[3]]);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
