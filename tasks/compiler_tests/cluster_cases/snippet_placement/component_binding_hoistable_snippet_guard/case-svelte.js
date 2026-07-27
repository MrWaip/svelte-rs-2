import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
import { CONST } from "x";
const foo = ($$anchor, a = $.noop) => {
	var span = root();
	var text = $.child(span);
	$.reset(span);
	$.template_effect(() => $.set_text(text, `${CONST ?? ""} ${a() ?? ""}`));
	$.append($$anchor, span);
};
var root = $.from_html(`<span> </span>`);
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
			foo($$anchor, () => 1);
		},
		$$slots: { default: true }
	});
	$.pop();
}
