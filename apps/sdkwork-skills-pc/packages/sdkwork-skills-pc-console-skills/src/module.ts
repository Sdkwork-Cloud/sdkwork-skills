/**
 * Console capability module metadata for the Skills PC application root.
 * The webserver host composes this module through its own module entries;
 * this metadata keeps the module's own PC root coherent.
 */
export const skillsConsoleModule = {
  id: 'console-skills',
  label: 'My Skills',
  surface: 'app-console',
  entries: [
    {
      resource: 'skills',
      label: 'My Skills',
      description: 'Skill packages owned by the authenticated user',
      order: 1,
    },
  ],
} as const;
