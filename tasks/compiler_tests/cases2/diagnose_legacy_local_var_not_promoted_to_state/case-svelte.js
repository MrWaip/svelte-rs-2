import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	let inserted = false;
	function shouldShow() {
		if (inserted) {
			return false;
		}
		inserted = true;
		return true;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var p = root_2();
				var text = $.child(p, true);
				$.reset(p);
				$.template_effect(() => $.set_text(text, $.get(item)));
				$.append($$anchor, p);
			};
			var d = $.derived(() => $.untrack(shouldShow));
			$.if(node_1, ($$render) => {
				if ($.get(d)) $$render(consequent);
			});
		}
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
