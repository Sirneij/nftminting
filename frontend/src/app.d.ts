// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
interface User {
	email: string;
	name: string;
	id: string;
	is_staff: boolean;
	is_active: boolean;
	provider: string;
	thumbnail: string;
	is_superuser: boolean;
	created_at: string;
	updated_at: string;
}
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: User;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
