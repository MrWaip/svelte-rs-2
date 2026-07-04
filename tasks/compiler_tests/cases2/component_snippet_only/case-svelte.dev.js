App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const row = $.wrap_snippet(App, function($$anchor, item = $.noop) {
			$.validate_snippet_args(...arguments);
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, item()));
			$.append($$anchor, span);
		});
		$.add_svelte_meta(() => Table($$anchor, {
			get items() {
				return $$props.data;
			},
			row,
			$$slots: { row: true }
		}), "component", App, 5, 0, { componentTag: "Table" });
	}
	return $.pop($$exports);
}
