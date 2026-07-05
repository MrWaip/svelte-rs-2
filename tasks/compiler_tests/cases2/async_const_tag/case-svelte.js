import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var data;
	var $$promises = $.run([async () => data = await fetch("/api")]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let value;
			var promises = $.run([() => $$promises[0].promise, () => value = $.derived(() => data.text)]);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(value)), void 0, void 0, [promises[1]]);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
