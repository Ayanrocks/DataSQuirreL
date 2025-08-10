import svelte from 'eslint-plugin-svelte';
import tseslint from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';

export default [
  // Ignore generated/artifact directories
  { ignores: ['debug/**', 'dist/**', 'build/**', 'public/**', 'node_modules/**'] },

  // Base Svelte recommendations (flat config)
  ...svelte.configs['flat/recommended'],

  // Allow TypeScript inside <script lang="ts"> in .svelte files
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parserOptions: {
        parser: tsParser,
        extraFileExtensions: ['.svelte'],
        sourceType: 'module',
      },
    },
    rules: {},
  },

  // TS files
  {
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      parser: tsParser,
      sourceType: 'module',
    },
    plugins: {
      '@typescript-eslint': tseslint,
    },
    rules: {},
  },
]; 