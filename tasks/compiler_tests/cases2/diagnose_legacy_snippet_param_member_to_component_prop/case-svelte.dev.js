import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item) => {
		const row = $.wrap_snippet(App, function($$anchor, $$arg0) {
			$.validate_snippet_args(...arguments);
			let value = () => ($$arg0?.()).value;
			value();
			$.add_svelte_meta(() => Child($$anchor, { get name() {
				return value(), $.untrack(() => value().name);
			} }), "component", App, 8, 2, { componentTag: "Child" });
		});
		$.add_svelte_meta(() => row($$anchor, () => ({ value: $.get(item) })), "render", App, 10, 1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
