<script lang="ts">
	import Button from '@papper-ppr/mordor/design/ds/components/Button';
	import { IcMQqFilled, IcMSimilarImagesFilled } from '@papper-ppr-ds/icons';
	import { BrowserQRCodeReader, BrowserQRCodeSvgWriter } from '@zxing/browser';

	import { base } from '$app/paths';
	import { apiClient } from '$gatewayService';
	import { GenericFormStepField as Field } from '$gatewayService/__queryTypes';
	import { withTimerPoll } from '$helpers/poll/index.svelte';
	import { eventBus } from '$lib/eventBus';
	import { ticketTransferNavigation } from '$shared/navigation/ticketTransfer';
	import { copyText } from '$shared/text';

	let output = $state('');
	let error = $state('');
	let svgContainer = $state<HTMLDivElement | undefined>();

	async function handlePaste() {
		try {
			error = '';
			output = '';

			const items = Array.from(await navigator.clipboard.read());
			const item = items?.[0];

			if (!item) throw new Error('lorem');
			if (!svgContainer) throw new Error('lorem');

			for (const type of item.types) {
				switch (type) {
					case 'text/plain': {
						const text = await (await item.getType(type)).text();

						output = text;

						const writer = new BrowserQRCodeSvgWriter();

						svgContainer.replaceChildren();
						writer.writeToDom(svgContainer, output, 200, 200);
						break;
					}

					case 'image/jpeg':
					case 'image/png': {
						const blob = await item.getType(type);
						const imageBitmap = await createImageBitmap(blob);

						const canvas = document.createElement('canvas');

						canvas.width = imageBitmap.width;
						canvas.height = imageBitmap.height;
						const ctx = canvas.getContext('2d')!;

						ctx.reset();
						ctx.drawImage(imageBitmap, 0, 0);

						const codeReader = new BrowserQRCodeReader();
						const result = codeReader.decodeFromCanvas(canvas);

						output = result.getText();

						const writer = new BrowserQRCodeSvgWriter();

						svgContainer.replaceChildren();
						writer.writeToDom(svgContainer, output, 200, 200);
						break;
					}
					default: {
						continue;
					}
				}
			}
		} catch (e) {
			output = '';
			error = String(e);
			eventBus.snackbarV2.alert('lorem');
		}
	}

	async function printActiveDraft() {
		if (!svgContainer) {
			return eventBus.snackbarV2.alert('lorem');
		}

		const result = await apiClient.ticketRequisites_init.call();
		const parts: { key: string; value: string }[] = [];

		if (result.isFail() && result.error.message.includes('NO_AUTH_X')) {
			return eventBus.snackbarV2.alert('lorem');
		}

		if (result.isFail()) {
			return eventBus.snackbarV2.alert('lorem');
		}

		for (const step of result.value.steps) {
			switch (step.field) {
				case Field.FIELD_A:
					parts.push({ key: 'fieldA', value: step.value });
					break;
				case Field.FIELD_B:
					parts.push({ key: 'fieldB', value: step.value });
					break;
				case Field.FIELD_C:
					parts.push({ key: 'fieldC', value: step.value });
					break;
				case Field.FIELD_D:
					parts.push({ key: 'fieldD', value: step.value });
					break;
				case Field.FIELD_E:
					parts.push({ key: 'fieldE', value: step.value });
					break;
				case Field.FIELD_F:
					parts.push({ key: 'fieldF', value: step.value });
					break;
				case Field.FIELD_G:
					parts.push({ key: 'fieldG', value: step.value });
					break;
				case Field.FIELD_H:
					parts.push({ key: 'fieldH', value: step.value });
					break;
				case Field.FIELD_I:
					parts.push({ key: 'fieldI', value: step.value });
					break;
				case Field.FIELD_J:
					parts.push({ key: 'fieldJ', value: step.value });
					break;
				case Field.FIELD_K:
					parts.push({ key: 'fieldK', value: step.value });
					break;
				case Field.FIELD_L:
					parts.push({ key: 'fieldL', value: step.value });
					break;
				case Field.FIELD_M:
					parts.push({ key: 'fieldM', value: step.value });
					break;
				case Field.FIELD_N:
					parts.push({ key: 'fieldN', value: step.value });
					break;
				case Field.FIELD_O:
					parts.push({ key: 'fieldO', value: step.value });
					break;
				case Field.FIELD_P:
					parts.push({ key: 'fieldP', value: step.value });
					break;
				case Field.FIELD_Q:
					parts.push({ key: 'fieldQ', value: step.value });
					break;
				case Field.UNKNOWN:
					continue;
			}
		}

		output = 'MAGIC_A|' + parts.map(({ key, value }) => `${key}=${value}`).join('|');

		const writer = new BrowserQRCodeSvgWriter();

		svgContainer.replaceChildren();
		writer.writeToDom(svgContainer, output, 200, 200);
	}

	async function handleCopy() {
		if (!output) return;

		copyText(output);

		eventBus.snackbarV2.success('lorem');
	}

	async function fakeScan() {
		const poll = withTimerPoll(apiClient.ticketRequisites_parseQr.call, {
			validate: (result) => {
				if (result.isFail() && result.error.hasError('WAIT_RESPONSE_FROM_SERVICE_X')) {
					return false;
				}

				return true;
			},
			interval: 500,
			timeoutMs: 13_000,
		});

		eventBus.progressStartLoader();

		const result = await poll({
			content: output,
			idempotencyKey: crypto.randomUUID(),
		});

		eventBus.progressStop();

		if (result.isFail() && result.error.message.includes('NO_AUTH_X')) {
			return eventBus.snackbarV2.alert('lorem');
		}

		if (!result.value?.draftId) {
			return eventBus.snackbarV2.alert('lorem');
		}

		const sourceURL = new URL(
			ticketTransferNavigation.buildTicketRequisitesUrl({
				continue: true,
				fromQr: true,
			})
		);

		const targetURL = new URL(`${base}/playground/MFGenericForm`, window.location.origin);

		for (const [key, value] of sourceURL.searchParams.entries()) {
			targetURL.searchParams.set(key, value);
		}

		window.open(targetURL, '__blank');
	}
</script>

<svelte:window onpaste={handlePaste} />

<div class="page">
	<div ppr-typo="tsHeadline600Medium">Lorem ipsum dolor sit amet</div>
	<div ppr-color="text-Secondary" ppr-typo="tsBody500Medium">
		Lorem ipsum dolor sit amet, consectetur adipiscing elit.
	</div>

	<div class="content">
		<pre ppr-typo="tsBody300XSmall" onclick={handleCopy}>{output || error}</pre>

		<div class="qr-container" bind:this={svgContainer}></div>
	</div>

	<div class="buttons">
		<Button
			icon={IcMSimilarImagesFilled}
			caption="lorem"
			color="actionSecondary"
			size="600"
			onclick={fakeScan}
			fullWidth
			disabled={!output}
		>
			lorem
		</Button>

		<Button
			icon={IcMQqFilled}
			caption="lorem"
			color="actionSecondary"
			size="600"
			onclick={printActiveDraft}
			fullWidth
		>
			lorem ipsum
		</Button>
	</div>
</div>

<style>
	.page {
		display: flex;
		width: 75rem;
		min-height: 37.5rem;
		flex-direction: column;
		padding: 2rem;
		border-radius: 2rem;
		margin: 2rem auto auto auto;
		background: var(--layerFloor1);
	}

	.content {
		display: grid;
		margin-top: 0.5rem;
		gap: 0.5rem;
		grid-template-columns: minmax(0, 1fr) minmax(0, 13.125rem);
	}

	pre {
		min-height: 12.5rem;
		padding: 0.5rem;
		border-radius: 1rem;
		margin: 0;

		background: var(--layerFloor0);
		cursor: pointer;
		white-space: pre-wrap;
		word-wrap: break-word;
	}

	.qr-container {
		width: 100%;
		min-height: 12.5rem;
		padding: 0.5rem;
		border-radius: 1rem;
		background: var(--layerFloor0);
	}

	pre:hover {
		background: var(--layerActiveFloor0);
	}

	.buttons {
		display: flex;
		margin-top: auto;
		gap: 0.5rem;
	}
</style>
