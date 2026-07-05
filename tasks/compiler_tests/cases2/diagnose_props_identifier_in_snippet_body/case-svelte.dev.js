App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const foo = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var fragment = $.comment();
		var node = $.first_child(fragment);
		$.add_svelte_meta(() => $.component(node, () => $$props.X, ($$anchor, props_X) => {
			props_X($$anchor, {});
		}), "component", App, 6, 1, { componentTag: "props.X" });
		$.append($$anchor, fragment);
	});
	let props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
