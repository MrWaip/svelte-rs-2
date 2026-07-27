import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>go</button>`, 1);
export default function App($$anchor) {
	let source = $.state($.proxy({
		x: 1,
		y: 2
	}));
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const a = $.derived(() => $.get(source).x);
			const computed_const = $.derived(() => {
				const { x, y } = $.get(source);
				return {
					x,
					y
				};
			});
			var p = root();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(computed_const).x ?? ""}${$.get(computed_const).y ?? ""}`));
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get(source)) $$render(consequent);
		});
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => $.set(source, {
		x: 3,
		y: 4
	}, true));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
