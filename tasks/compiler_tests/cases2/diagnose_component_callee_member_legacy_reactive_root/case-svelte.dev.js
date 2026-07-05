import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const Holder = $.mutable_source();
	let flag = $.prop($$props, "flag", 8);
	$.legacy_pre_effect(() => Inner, () => {
		$.set(Holder, { component: Inner });
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.add_svelte_meta(() => $.get(Holder).component($$anchor, {}), "component", App, 8, 1, { componentTag: "Holder.component" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (flag()) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
