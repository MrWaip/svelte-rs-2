import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const foo = ($$anchor, a = $.noop) => {
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${$$props.items ?? ""} ${a() ?? ""}`));
		$.append($$anchor, span);
	};
	let ref = $.prop($$props, "ref", 15);
	Child($$anchor, {
		get ref() {
			return ref();
		},
		set ref($$value) {
			ref($$value);
		},
		children: ($$anchor, $$slotProps) => {
			foo($$anchor, () => 2);
		},
		$$slots: { default: true }
	});
	$.pop();
}
