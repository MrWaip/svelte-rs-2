import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let depth = $.prop($$props, "depth", 8, 0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				let $0 = $.derived_safe_equal(() => depth() - 1);
				App(node_1, {
					get depth() {
						return $.get($0);
					},
					children: $.invalid_default_snippet,
					$$slots: { default: ($$anchor, $$slotProps) => {
						const item = $.derived_safe_equal(() => $$slotProps.item);
						const index = $.derived_safe_equal(() => $$slotProps.index);
						var p = root();
						var text = $.child(p);
						$.reset(p);
						$.template_effect(() => $.set_text(text, `${$.get(item) ?? ""} ${$.get(index) ?? ""}`));
						$.append($$anchor, p);
					} }
				});
			}
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (depth() > 0) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
