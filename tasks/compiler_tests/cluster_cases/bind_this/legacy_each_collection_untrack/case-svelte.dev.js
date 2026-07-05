import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root = $.add_locations($.from_html(`<!> <div> </div>`, 1), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let foo = $.prop($$props, "foo", 28, () => []);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 0, () => Array(3), $.index, ($$anchor, _, i) => {
		var fragment_1 = root();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.bind_this(Foo(node_1, { $$legacy: true }), ($$value, i) => $$ownership_validator.mutation(null, ["foo", i], foo(foo()[i] = $$value, true), 7, 17), (i) => foo()?.[i], () => [i]), "component", App, 7, 1, { componentTag: "Foo" });
		var div = $.sibling(node_1, 2);
		var text = $.child(div);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `${i} has foo: ${($.deep_read_state(foo()), i, $.untrack(() => !!foo()[i])) ?? ""}`));
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
