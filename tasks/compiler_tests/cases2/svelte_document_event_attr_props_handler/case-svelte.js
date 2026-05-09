import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.event("click", $.document, function(...$$args) {
		$$props.onClick?.apply(this, $$args);
	});
	$.event("custom-event", $.document, function(...$$args) {
		$$props.onCustom?.apply(this, $$args);
	});
}
