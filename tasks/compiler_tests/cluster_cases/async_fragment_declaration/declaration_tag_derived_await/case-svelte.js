import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let g;
			var promises = $.run([async () => g = await $.async_derived(() => Promise.resolve($.get(n)))]);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(g)), void 0, void 0, [promises[0]]);
			$.append($$anchor, p);
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
