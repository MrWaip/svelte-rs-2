import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const Holder = $.mutable_source();
	let flag = $.prop($$props, "flag", 8);
	$.legacy_pre_effect(() => Inner, () => {
		$.set(Holder, { component: Inner });
	});
	$.legacy_pre_effect_reset();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.get(Holder).component($$anchor, {});
		};
		$.if(node, ($$render) => {
			if (flag()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
}
