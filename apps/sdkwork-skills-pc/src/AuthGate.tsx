import { type ReactNode, useEffect, useMemo, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { SdkworkIamAuthRoutes } from '@sdkwork/auth-pc-react';

import {
  resolveSdkworkSkillsPcAuthAppearance,
  resolveSdkworkSkillsPcAuthLocale,
  resolveSdkworkSkillsPcAuthRuntimeConfig,
} from './bootstrap/authConfig';
import type { SdkworkSkillsPcRuntime } from './bootstrap/runtime';
import {
  hasSdkworkSkillsPcAuthenticatedSession,
  resolveSdkworkSkillsPcAuthGateDecision,
} from './authGateLogic';

export interface AuthGateProps {
  children: ReactNode;
  runtime: SdkworkSkillsPcRuntime;
}

export function AuthGate({ children, runtime }: AuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [snapshot, setSnapshot] = useState(() => runtime.session.getSnapshot());

  useEffect(() => runtime.session.subscribe(setSnapshot), [runtime.session]);

  const decision = useMemo(
    () =>
      resolveSdkworkSkillsPcAuthGateDecision({
        hasSession: hasSdkworkSkillsPcAuthenticatedSession(snapshot),
        homePath: '/skills-hub',
        location,
      }),
    [location, snapshot],
  );

  useEffect(() => {
    if (decision.kind !== 'redirect') {
      return;
    }
    navigate(decision.to, { replace: true });
  }, [decision, navigate]);

  if (decision.kind === 'redirect') {
    return null;
  }

  if (decision.kind === 'auth-route') {
    const authProps = {
      appearance: resolveSdkworkSkillsPcAuthAppearance(),
      basePath: '/auth',
      getRuntime: () => runtime.iamRuntime,
      homePath: '/skills-hub',
      locale: resolveSdkworkSkillsPcAuthLocale(runtime.config.i18n.defaultLocale),
      runtimeConfig: resolveSdkworkSkillsPcAuthRuntimeConfig(),
      viewportMode: 'flow' as const,
    };

    return (
      <SdkworkIamAuthRoutes
        {...(authProps as unknown as Parameters<typeof SdkworkIamAuthRoutes>[0])}
      />
    );
  }

  return <>{children}</>;
}
