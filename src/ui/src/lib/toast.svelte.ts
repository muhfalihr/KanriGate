export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface Toast {
	id: string;
	message: string;
	type: ToastType;
}

export function createToast() {
	let toasts = $state<Toast[]>([]);

	function add(message: string, type: ToastType = 'info', duration = 3000) {
		const id = crypto.randomUUID();
		toasts = [...toasts, { id, message, type }];

		if (duration > 0) {
			setTimeout(() => {
				remove(id);
			}, duration);
		}
	}

	function remove(id: string) {
		toasts = toasts.filter((t) => t.id !== id);
	}

	return {
		get current() {
			return toasts;
		},
		add,
		remove,
		success: (m: string) => add(m, 'success'),
		error: (m: string) => add(m, 'error'),
		info: (m: string) => add(m, 'info'),
		warning: (m: string) => add(m, 'warning')
	};
}

export const toast = createToast();
