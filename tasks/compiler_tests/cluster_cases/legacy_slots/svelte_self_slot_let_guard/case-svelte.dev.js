import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let depth = $.prop($$props, "depth", 8, 0);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			{
				let $0 = $.derived_safe_equal(() => depth() - 1);
				$.add_svelte_meta(() => App(node_1, {
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
				}), "component", App, 6, 1, { componentTag: "svelte:self" });
			}
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (depth() > 0) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
