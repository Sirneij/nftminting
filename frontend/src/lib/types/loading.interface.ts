type Status = 'IDLE' | 'LOADING' | 'NAVIGATING';

export interface Loading {
	status: Status;
	message: string;
}
