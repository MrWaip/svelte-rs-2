App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { foo } from "./x.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Comp($$anchor, { get p() {
		return foo;
	} }), "component", App, 7, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
