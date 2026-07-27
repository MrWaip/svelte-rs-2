App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 15]]);
var root_1 = $.add_locations($.from_html(`<em> </em>`), App[$.FILENAME], [[7, 15]]);
var root_2 = $.add_locations($.from_html(`<!><!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	const a = $.wrap_snippet(App, function($$anchor, p = $.noop) {
		$.validate_snippet_args(...arguments);
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${$$props.items ?? ""} ${p() ?? ""}`));
		$.append($$anchor, span);
	});
	const b = $.wrap_snippet(App, function($$anchor, q = $.noop) {
		$.validate_snippet_args(...arguments);
		var em = root_1();
		var text_1 = $.child(em);
		$.reset(em);
		$.template_effect(() => $.set_text(text_1, `${$$props.items ?? ""} ${q() ?? ""}`));
		$.append($$anchor, em);
	});
	let ref = $.prop($$props, "ref", 15);
	var $$exports = { ...$.legacy_api() };
	{
		$$ownership_validator.binding("ref", Child, ref);
		$.add_svelte_meta(() => Child($$anchor, {
			get ref() {
				return ref();
			},
			set ref($$value) {
				ref($$value);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var fragment_1 = root_2();
				var node = $.first_child(fragment_1);
				$.add_svelte_meta(() => a(node, () => 1), "render", App, 9, 16);
				var node_1 = $.sibling(node);
				$.add_svelte_meta(() => b(node_1, () => 2), "render", App, 9, 30);
				$.append($$anchor, fragment_1);
			}),
			$$slots: { default: true }
		}), "component", App, 9, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
