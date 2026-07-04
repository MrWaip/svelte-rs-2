App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let store = $.prop($$props, "store", 15);
	var $$exports = { ...$.legacy_api() };
	$.validate_binding("bind:value={store.inner.value}", [], () => store().inner, () => "value", 6, 6);
	{
		$$ownership_validator.binding("store", Comp, () => store().inner.value);
		$.add_svelte_meta(() => Comp($$anchor, {
			get value() {
				return store().inner.value;
			},
			set value($$value) {
				$$ownership_validator.mutation("store", [
					"store",
					"inner",
					"value"
				], store(store().inner.value = $$value, true), 6, 18);
			}
		}), "component", App, 6, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
