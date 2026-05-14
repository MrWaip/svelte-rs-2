import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let cond = $.prop($$props, "cond", 8, false);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const xs = $.derived_safe_equal(() => [cond() ? 1 : 2]);
			const ys = $.derived_safe_equal(() => [3]);
			const all = $.derived_safe_equal(() => ($.deep_read_state($.get(xs)), $.deep_read_state($.get(ys)), $.untrack(() => [...$.get(xs), ...$.get(ys)])));
			var text = $.text();
			$.template_effect(() => $.set_text(text, ($.deep_read_state($.get(all)), $.untrack(() => $.get(all).length))));
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if (cond()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
