import { createContext, useContext, type ReactNode } from 'react';

import { createSkillsClients, type SkillsClients } from './clients';

const SkillsClientsContext = createContext<SkillsClients | null>(null);

export function SkillsClientsProvider({
  clients,
  children,
}: {
  clients?: SkillsClients;
  children: ReactNode;
}) {
  const value = clients ?? createSkillsClients();
  return <SkillsClientsContext.Provider value={value}>{children}</SkillsClientsContext.Provider>;
}

export function useSkillsClients(): SkillsClients {
  const clients = useContext(SkillsClientsContext);
  if (!clients) {
    throw new Error('useSkillsClients must be used within SkillsClientsProvider');
  }
  return clients;
}
