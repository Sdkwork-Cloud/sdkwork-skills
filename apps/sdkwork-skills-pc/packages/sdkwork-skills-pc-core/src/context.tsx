import { createContext, useContext, type ReactNode } from 'react';

import type { SkillsClients } from './clients';

const SkillsClientsContext = createContext<SkillsClients | null>(null);

export function SkillsClientsProvider({
  clients,
  children,
}: {
  clients: SkillsClients;
  children: ReactNode;
}) {
  return <SkillsClientsContext.Provider value={clients}>{children}</SkillsClientsContext.Provider>;
}

export function useSkillsClients(): SkillsClients {
  const clients = useContext(SkillsClientsContext);
  if (!clients) {
    throw new Error('useSkillsClients must be used within SkillsClientsProvider');
  }
  return clients;
}
