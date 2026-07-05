App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { isNotNil } from "es-toolkit";
import TransferFormLoading from "$apps/MFInternationalTransfersByPhone/views/TransferFormLoading.svelte";
import { contextifiedStore } from "../store/RootStore.svelte";
import { View } from "../store/views";
import TransferFormLoaded from "./TransferFormLoaded.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const rootStore = contextifiedStore.getStore();
		const { payoffLazy } = rootStore.navigation.useViewPayload(View.TransferForm);
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (isNotNil(payoffLazy.data)) {
				$$renderer.push("<!--[0-->");
				TransferFormLoaded($$renderer, {
					get payoffStore() {
						return payoffLazy.data;
					},
					set payoffStore($$value) {
						payoffLazy.data = $$value;
						$$settled = false;
					}
				});
			} else {
				$$renderer.push("<!--[-1-->");
				TransferFormLoading($$renderer, {});
			}
			$$renderer.push(`<!--]-->`);
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
