import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const item = getItem();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.tag($.derived_safe_equal(() => {
				const { title, status } = item;
				return {
					title,
					status
				};
			}), "[@const]");
			$.get(computed_const);
			$.add_svelte_meta(() => Child($$anchor, {
				get status() {
					return $.get(computed_const).status;
				},
				children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
					$.next();
					var text = $.text();
					$.template_effect(() => $.set_text(text, $.get(computed_const).title));
					$.append($$anchor, text);
				}),
				$$slots: { default: true }
			}), "component", App, 10, 2, { componentTag: "Child" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (item) $$render(consequent);
		}), "if", App, 8, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
