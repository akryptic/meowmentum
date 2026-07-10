type GDriveStatus = 'connected' | 'connecting' | 'disconnected';

export enum DriveProvider {
	NONE,
	GOOGLE_DRIVE,
	ONE_DRIVE
}

interface IAppState {
	driveProvider: DriveProvider;
	driveStatus: GDriveStatus;
}

class AppState implements IAppState {
	#driveStatus = $state<GDriveStatus>('disconnected');
	driveProvider = $state<DriveProvider>(DriveProvider.NONE);

	get driveStatus() {
		return this.#driveStatus;
	}

	set driveStatus(status: GDriveStatus) {
		this.#driveStatus = status;
	}
}

// Singleton
const appState = new AppState();
export default appState;
