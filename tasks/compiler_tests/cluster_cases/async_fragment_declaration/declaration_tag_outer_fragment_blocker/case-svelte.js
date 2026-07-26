import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><span> </span></div>`);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let outer;
			var promises = $.run([async () => outer = await $.async_derived(() => Promise.resolve($.get(n)))]);
			var div = root();
			{
				let inner;
				var promises_1 = $.run([() => promises[0].promise, () => inner = $.derived(() => `v${$.get(outer)}`)]);
				var span = $.child(div);
				var text = $.child(span, true);
				$.reset(span);
				$.reset(div);
				$.template_effect(() => $.set_text(text, $.get(inner)), void 0, void 0, [promises_1[1]]);
			}
			$.append($$anchor, div);
		};
		$.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		});
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
