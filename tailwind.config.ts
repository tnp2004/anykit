import { join } from 'path';
import type { Config } from 'tailwindcss';
import { skeleton } from '@skeletonlabs/tw-plugin';

const config =  {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}',
  join(require.resolve(
    '@skeletonlabs/skeleton'),
    '../**/*.{html,js,svelte,ts}'
  )],
  theme: {
    extend: {
      fontFamily: {
        lexend: ['Lexend', 'sans-serif'],
      },
    },
  },
  plugins: [skeleton],
} satisfies Config;

export default config;