type Submit = { type: 'idle' };

export class Store {
	value: Submit;

	constructor() {
		this.value = $state.raw({ type: 'idle' });
	}
}
