import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>bump</button>`, 1);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let n = $.state(1);
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			let s;
			var promises = $.run([async () => s = $.state($.proxy(await Promise.resolve($.get(n))))]);
			var fragment_1 = root();
			var p = $.first_child(fragment_1);
			var text = $.child(p, true);
			$.reset(p);
			var button = $.sibling(p, 2);
			$.template_effect(() => $.set_text(text, $.get(s)), void 0, void 0, [promises[0]]);
			$.delegated("click", button, () => $.update(s));
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		});
	}
	var button_1 = $.sibling(node, 2);
	$.delegated("click", button_1, () => $.update(n));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
