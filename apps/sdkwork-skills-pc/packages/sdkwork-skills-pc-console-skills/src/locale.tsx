import { createContext, useCallback, useContext, type ReactNode } from "react";
import {
  normalizeSkillsConsoleLocale,
  translateSkillsConsole,
  type SkillsConsoleLocale,
  type SkillsConsoleMessageKey,
} from "./i18n.ts";

const SkillsConsoleLocaleContext = createContext<SkillsConsoleLocale>("en-US");

export function SkillsConsoleLocaleProvider({
  children,
  locale,
}: {
  children: ReactNode;
  locale?: string | null;
}) {
  return (
    <SkillsConsoleLocaleContext.Provider value={normalizeSkillsConsoleLocale(locale)}>
      {children}
    </SkillsConsoleLocaleContext.Provider>
  );
}

export function useSkillsConsoleLocale(): SkillsConsoleLocale {
  return useContext(SkillsConsoleLocaleContext);
}

export function useSkillsConsoleT() {
  const locale = useSkillsConsoleLocale();
  return useCallback(
    (key: SkillsConsoleMessageKey, values: Record<string, string | number> = {}) =>
      translateSkillsConsole(locale, key, values),
    [locale],
  );
}
