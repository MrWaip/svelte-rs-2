App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const prop = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, foo));
			$.append($$anchor, text);
		});
		$.add_svelte_meta(() => Comp($$anchor, {
			prop,
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				const foo = $.tag($.derived(() => $$props.x * 2), "foo");
				$.get(foo);
			}),
			$$slots: {
				prop: true,
				default: true
			}
		}), "component", App, 4, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
