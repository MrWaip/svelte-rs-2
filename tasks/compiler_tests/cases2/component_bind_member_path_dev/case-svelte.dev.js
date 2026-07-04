App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let store = $.tag_proxy($.proxy({ inner: { value: 0 } }), "store");
	var $$exports = { ...$.legacy_api() };
	$.validate_binding("bind:value={store.inner.value}", [], () => store.inner, () => "value", 6, 6);
	$.add_svelte_meta(() => Comp($$anchor, {
		get value() {
			return store.inner.value;
		},
		set value($$value) {
			store.inner.value = $$value;
		}
	}), "component", App, 6, 0, { componentTag: "Comp" });
	return $.pop($$exports);
}
