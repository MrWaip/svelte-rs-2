import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<em> </em>`);
var root_2 = $.from_html(`<!><!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const a = ($$anchor, p = $.noop) => {
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${$$props.items ?? ""} ${p() ?? ""}`));
		$.append($$anchor, span);
	};
	const b = ($$anchor, q = $.noop) => {
		var em = root_1();
		var text_1 = $.child(em);
		$.reset(em);
		$.template_effect(() => $.set_text(text_1, `${$$props.items ?? ""} ${q() ?? ""}`));
		$.append($$anchor, em);
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
			var fragment_1 = root_2();
			var node = $.first_child(fragment_1);
			a(node, () => 1);
			var node_1 = $.sibling(node);
			b(node_1, () => 2);
			$.append($$anchor, fragment_1);
		},
		$$slots: { default: true }
	});
	$.pop();
}
