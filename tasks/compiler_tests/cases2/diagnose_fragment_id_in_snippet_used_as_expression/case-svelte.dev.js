App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const body = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				$.add_svelte_meta(() => $.component(node_1, () => $$props.Inner, ($$anchor, props_Inner) => {
					props_Inner($$anchor, {});
				}), "component", App, 8, 2, { componentTag: "props.Inner" });
				$.append($$anchor, fragment_1);
			};
			$.add_svelte_meta(() => $.if(node, ($$render) => {
				if ($$props.Inner) $$render(consequent);
			}), "if", App, 7, 1);
		}
		$.append($$anchor, fragment);
	});
	let props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(() => $$props.show ? body : undefined);
		$.add_svelte_meta(() => Child($$anchor, { get icon() {
			return $.get($0);
		} }), "component", App, 12, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
