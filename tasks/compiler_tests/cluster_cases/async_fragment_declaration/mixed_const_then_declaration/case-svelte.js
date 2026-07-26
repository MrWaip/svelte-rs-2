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
			let c;
			var promises = $.run([async () => a = (await $.save($.async_derived(async () => (await $.save(Promise.resolve(n)))())))(), () => {
				b = $.derived(() => $.get(a) * 2);
				c = $.get(a) + 1;
			}]);
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `${$.get(b) ?? ""} ${c}`), void 0, void 0, [promises[1]]);
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if (n) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
