import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<span> </span>`);
export default function App($$anchor, $$props) {
	{
		const row = ($$anchor, item = $.noop) => {
			var span = root_1();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, item()));
			$.append($$anchor, span);
		};
		Table($$anchor, {
			get items() {
				return $$props.data;
			},
			row,
			$$slots: { row: true }
		});
	}
}
