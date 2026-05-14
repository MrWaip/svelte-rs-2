import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<!> <div>tail</div>`, 1);
var root = $.from_html(`<li><!></li>`);
export default function App($$anchor, $$props) {
	let show = $.prop($$props, "show", 8);
	let value = $.prop($$props, "value", 8);
	var li = root();
	var node = $.child(li);
	$.slot(node, $$props, "item", {}, ($$anchor) => {
		var fragment = root_1();
		var node_1 = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var span = root_2();
				var text = $.child(span, true);
				$.reset(span);
				$.template_effect(() => $.set_text(text, value()));
				$.append($$anchor, span);
			};
			$.if(node_1, ($$render) => {
				if (show()) $$render(consequent);
			});
		}
		$.next(2);
		$.append($$anchor, fragment);
	});
	$.reset(li);
	$.append($$anchor, li);
}
