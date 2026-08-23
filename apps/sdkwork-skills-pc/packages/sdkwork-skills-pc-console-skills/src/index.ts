export { CreateSkillPage, EditSkillPage, MySkillsPage } from './pages';
export {
  formatSkillsStatusLocalized,
  formatSkillsVisibilityLocalized,
  normalizeSkillsConsoleLocale,
  translateSkillsConsole,
  type SkillsConsoleLocale,
  type SkillsConsoleMessageKey,
} from './i18n.ts';
export {
  SkillsConsoleLocaleProvider,
  useSkillsConsoleLocale,
  useSkillsConsoleT,
} from './locale.tsx';
