App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let comp = $.tag($.state(void 0), "comp");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => $.bind_this(Comp($$anchor, {}), ($$value) => $.set(comp, $$value, true), () => $.get(comp)), "component", App, 5, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
