import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor) {
	const badge = ($$anchor, text = $.noop) => {
		var span = root_1();
		$.set_class(span, 1, "badge svelte-yczv4j", null, {}, { primary: variant === "primary" });
		var text_1 = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text_1, text()));
		$.append($$anchor, span);
	};
	let variant = "primary";
	badge($$anchor, () => "hi");
}
