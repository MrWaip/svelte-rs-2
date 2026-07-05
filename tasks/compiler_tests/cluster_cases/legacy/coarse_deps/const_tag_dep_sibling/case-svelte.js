import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let obj = $.prop($$props, "obj", 8);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const name = $.derived_safe_equal(() => ($.deep_read_state(obj()), $.untrack(() => obj().name)));
			const len = $.derived_safe_equal(() => ($.deep_read_state($.get(name)), $.untrack(() => $.get(name).length)));
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""}: ${$.get(len) ?? ""}`));
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if (obj()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
