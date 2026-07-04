App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { obj } from "./x.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const y = obj.prop;
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Comp($$anchor, { get p() {
		return y;
	} }), "component", App, 8, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
