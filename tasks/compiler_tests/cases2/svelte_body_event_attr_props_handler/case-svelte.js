import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.event("click", $.document.body, function(...$$args) {
		$$props.onClick?.apply(this, $$args);
	});
	$.event("custom-event", $.document.body, function(...$$args) {
		$$props.onCustom?.apply(this, $$args);
	});
}
