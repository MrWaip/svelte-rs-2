import * as $ from "svelte/internal/client";
var root = $.from_html(`<span></span>`);
export default function App($$anchor) {
	Component($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var span = root();
			{
				const children = ($$anchor, $$arg0) => {
					let with_prop = () => ($$arg0?.()).with_prop;
					$.next();
					var text = $.text();
					$.template_effect(() => $.set_text(text, `txt ${with_prop() ?? ""}`));
					$.append($$anchor, text);
				};
			}
			$.append($$anchor, span);
		},
		$$slots: { default: true }
	});
}
