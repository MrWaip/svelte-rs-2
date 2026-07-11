import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let value = $.prop($$props, "value", 12);
	var $$exports = { ...$.legacy_api() };
	var bind_get = () => value();
	var bind_set = (v) => value(v?.trim());
	$.add_svelte_meta(() => Comp($$anchor, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		},
		$$legacy: true
	}), "component", App, 8, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
