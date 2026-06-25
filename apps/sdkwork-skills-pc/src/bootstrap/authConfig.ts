import type { SdkworkAuthRuntimeConfig } from '@sdkwork/auth-pc-react';

export interface SdkworkSkillsPcAuthAppearanceConfig {
  asidePanelClassName?: string;
  bodyClassName?: string;
  contentContainerClassName?: string;
  pageClassName?: string;
  qrFrameClassName?: string;
  shellClassName?: string;
  slotProps?: {
    background?: { className?: string };
    page?: { className?: string };
    shell?: { className?: string };
  };
  theme?: Record<string, string>;
}

export type SdkworkSkillsPcAuthRuntimeConfig = SdkworkAuthRuntimeConfig;

const SKILLS_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: true,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: true,
  phoneRegistrationVerificationRequired: false,
};

export function resolveSdkworkSkillsPcAuthRuntimeConfig(): SdkworkSkillsPcAuthRuntimeConfig {
  return {
    leftRailMode: 'qr-only',
    loginMethods: ['password', 'emailCode', 'phoneCode'],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: ['email', 'phone'],
    registerMethods: ['email', 'phone'],
    verificationPolicy: SKILLS_VERIFICATION_POLICY,
  };
}

export function resolveSdkworkSkillsPcAuthAppearance(): SdkworkSkillsPcAuthAppearanceConfig {
  return {
    asidePanelClassName: 'sdkwork-skills-pc-auth-aside-panel',
    bodyClassName: 'sdkwork-skills-pc-auth-body',
    contentContainerClassName: 'sdkwork-skills-pc-auth-content',
    pageClassName: 'sdkwork-skills-pc-auth-page',
    qrFrameClassName: 'sdkwork-skills-pc-auth-qr-frame',
    shellClassName: 'sdkwork-skills-pc-auth-card-shell',
    slotProps: {
      background: {
        className: 'sdkwork-skills-pc-auth-background',
      },
      page: {
        className: 'sdkwork-skills-pc-auth-page',
      },
      shell: {
        className: 'sdkwork-skills-pc-auth-card-shell',
      },
    },
  };
}

export function resolveSdkworkSkillsPcAuthLocale(defaultLocale: string): string {
  return defaultLocale;
}
