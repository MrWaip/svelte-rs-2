App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Foo($$anchor, { onlyOrder: true }), "component", App, 5, 0, { componentTag: "Foo" });
	return $.pop($$exports);
}
