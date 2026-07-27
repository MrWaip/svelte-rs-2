import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let ref = $.prop($$props, "ref", 15);
	Child($$anchor, {
		get ref() {
			return ref();
		},
		set ref($$value) {
			ref($$value);
		},
		children: ($$anchor, $$slotProps) => {
			$.next();
			var text = $.text();
			$.template_effect(() => $.set_text(text, $$props.items));
			$.append($$anchor, text);
		},
		$$slots: { default: true }
	});
	$.pop();
}
