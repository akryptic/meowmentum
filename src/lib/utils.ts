export const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export function isPathActive(currentPath: string, path: string): boolean {
	// Home only matches the exact root.
	if (path === '/') {
		return currentPath === '/';
	}

	return currentPath === path || currentPath.startsWith(path + '/');
}