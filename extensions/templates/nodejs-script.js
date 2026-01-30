#!/usr/bin/env node
/**
 * Example Node.js script extension
 * Make executable: chmod +x nodejs-script.js
 */

const fs = require('fs');
const path = require('path');

// Get context from environment variables
const repoRoot = process.env.DEVKIT_REPO_ROOT;
const isQuiet = process.env.DEVKIT_QUIET === '1';
const hasDocker = process.env.DEVKIT_FEATURE_DOCKER === '1';
const hasGit = process.env.DEVKIT_FEATURE_GIT === '1';
const hasCargo = process.env.DEVKIT_FEATURE_CARGO === '1';
const hasNode = process.env.DEVKIT_FEATURE_NODE === '1';
const hasDatabase = process.env.DEVKIT_FEATURE_DATABASE === '1';

console.log(`📦 Node.js script running in ${repoRoot}`);

if (!isQuiet) {
  console.log(`✓ Docker: ${hasDocker ? 'available' : 'not available'}`);
  console.log(`✓ Git: ${hasGit ? 'available' : 'not available'}`);
  console.log(`✓ Cargo: ${hasCargo ? 'available' : 'not available'}`);
  console.log(`✓ Node.js: ${hasNode ? 'available' : 'not available'}`);
  console.log(`✓ Database: ${hasDatabase ? 'configured' : 'not configured'}`);
}

// Your custom logic here
console.log('Running custom Node.js workflow...');

// Example: check if package.json exists
const packageJsonPath = path.join(repoRoot, 'package.json');
if (fs.existsSync(packageJsonPath)) {
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  console.log(`Project: ${packageJson.name}@${packageJson.version}`);
}

console.log('✓ Done!');
process.exit(0);
