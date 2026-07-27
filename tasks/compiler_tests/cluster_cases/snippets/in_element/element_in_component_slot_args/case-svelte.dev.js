App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span></span>`), App[$.FILENAME], [[1, 11]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Component($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var span = root();
			{
				const children = $.wrap_snippet(App, function($$anchor, $$arg0) {
					$.validate_snippet_args(...arguments);
					let with_prop = () => ($$arg0?.()).with_prop;
					with_prop();
					$.next();
					var text = $.text();
					$.template_effect(() => $.set_text(text, `txt ${with_prop() ?? ""}`));
					$.append($$anchor, text);
				});
			}
			$.append($$anchor, span);
		}),
		$$slots: { default: true }
	}), "component", App, 1, 0, { componentTag: "Component" });
	return $.pop($$exports);
}
