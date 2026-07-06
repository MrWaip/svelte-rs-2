DepositMethod[$.FILENAME] = "DepositMethod.svelte";
import * as $ from "svelte/internal/server";
function DepositMethod($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { $$slots, $$events, ...props } = $$props;
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 6, 0);
		$$renderer.push(`${$.escape(props.title)}</p>`);
		$.pop_element();
	}, DepositMethod);
}
DepositMethod.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default DepositMethod;
