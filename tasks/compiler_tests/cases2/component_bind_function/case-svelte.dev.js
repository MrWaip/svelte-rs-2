App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state(0), "value");
	var $$exports = { ...$.legacy_api() };
	var bind_get = () => $.get(value);
	var bind_set = (v) => $.set(value, v, true);
	$.add_svelte_meta(() => Comp($$anchor, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		}
	}), "component", App, 6, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
