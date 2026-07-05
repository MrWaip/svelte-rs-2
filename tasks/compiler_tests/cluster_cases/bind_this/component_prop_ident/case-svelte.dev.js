import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => $.bind_this(Foo($$anchor, { $$legacy: true }), ($$value) => foo($$value), () => foo()), "component", App, 5, 0, { componentTag: "Foo" });
	return $.pop($$exports);
}
