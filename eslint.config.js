import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';
import globals from 'globals';

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
  js.configs.recommended,
  ...svelte.configs['flat/recommended'],
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.es2022 },
    },
  },
  {
    files: ['**/*.ts', '**/*.svelte.ts'],
    plugins: { '@typescript-eslint': ts },
    languageOptions: {
      parser: tsParser,
      globals: {
        $state: 'readonly',
        $derived: 'readonly',
        $effect: 'readonly',
        $props: 'readonly',
        $bindable: 'readonly',
        $inspect: 'readonly',
      },
    },
    rules: {
      '@typescript-eslint/no-explicit-any': 'warn',
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
    },
  },
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parser: svelteParser,
      parserOptions: { parser: tsParser },
      globals: {
        // Svelte 5 runes — compiler-injected globals
        $state: 'readonly',
        $derived: 'readonly',
        $effect: 'readonly',
        $props: 'readonly',
        $bindable: 'readonly',
        $inspect: 'readonly',
      },
    },
    rules: {
      'svelte/no-at-html-tags': 'error',
      'no-unused-vars': 'off',
    },
  },
  {
    ignores: ['build/', '.svelte-kit/', 'src-tauri/'],
  },
];
