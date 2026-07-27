import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	const foo = ($$anchor, a = $.noop) => {
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${$$props.items ?? ""} ${a() ?? ""}`));
		$.append($$anchor, span);
	};
	Child($$anchor, {
		children: ($$anchor, $$slotProps) => {
			foo($$anchor, () => 1);
		},
		$$slots: { default: true }
	});
}
