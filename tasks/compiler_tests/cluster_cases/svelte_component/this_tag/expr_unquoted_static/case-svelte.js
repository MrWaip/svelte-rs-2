import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => Foo, ($$anchor, $$component) => {
		$$component($$anchor, {});
	});
	$.append($$anchor, fragment);
}
