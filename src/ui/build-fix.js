import fs from 'fs';
import gracefulFs from 'graceful-fs';
import { execSync } from 'child_process';

gracefulFs.gracefulify(fs);

console.log('Patched fs with graceful-fs. Starting build...');

try {
	execSync('vite build', { stdio: 'inherit' });
} catch (error) {
	process.exit(1);
}
