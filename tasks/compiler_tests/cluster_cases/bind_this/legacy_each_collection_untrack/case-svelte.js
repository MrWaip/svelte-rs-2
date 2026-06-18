import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root_1 = $.from_html(`<!> <div> </div>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.prop($$props, "foo", 28, () => []);
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 0, () => Array(3), $.index, ($$anchor, _, i) => {
		var fragment_1 = root_1();
		var node_1 = $.first_child(fragment_1);
		$.bind_this(Foo(node_1, { $$legacy: true }), ($$value, i) => foo(foo()[i] = $$value, true), (i) => foo()?.[i], () => [i]);
		var div = $.sibling(node_1, 2);
		var text = $.child(div);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `${i} has foo: ${($.deep_read_state(foo()), i, $.untrack(() => !!foo()[i])) ?? ""}`));
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	$.pop();
}
